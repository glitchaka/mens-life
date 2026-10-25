#include "mspa/psychology/profile.hpp"
#include "mspa/base/rng.hpp"
#include "mspa/psychology/personality.hpp"
#include <algorithm>
#include <cmath>
#include <set>
namespace mspa {namespace {
float clamp01(float x){return std::clamp(x,0.0f,1.0f);}float clamp11(float x){return std::clamp(x,-1.0f,1.0f);}bool finite(float x){return std::isfinite(x);}
struct WeightedValue{float sum{0};float weight{0};};
struct WeightedGoal{float priority_sum{0};float commitment_sum{0};float weight{0};};
}
Result<void> ProfileRegistry::add(PsychologicalProfile profile){
    if(!profile.id.valid()||profile.name.empty())return make_error(ErrorCode::InvalidArgument,"profile requires a valid id and non-empty name");
    if(profiles_.find(profile.id)!=profiles_.end())return make_error(ErrorCode::InvalidArgument,"duplicate psychological profile id");
    auto normalized=PersonalitySystem{}.normalize(profile.personality);if(!normalized)return *normalized.error_if();
    if(!finite(profile.baseline_mood.valence)||!finite(profile.baseline_mood.activation))return make_error(ErrorCode::InvalidArgument,"profile mood contains non-finite value");
    profile.baseline_mood.valence=clamp11(profile.baseline_mood.valence);profile.baseline_mood.activation=clamp01(profile.baseline_mood.activation);profile.baseline_mood.updated_at={};
    std::set<ConceptId> value_ids;for(auto&v:profile.values){if(!v.concept_id.valid()||!finite(v.intensity))return make_error(ErrorCode::InvalidArgument,"profile value is invalid");if(!value_ids.insert(v.concept_id).second)return make_error(ErrorCode::InvalidArgument,"profile contains duplicate value concept");v.intensity=clamp01(v.intensity);}
    std::set<ConceptId> goal_ids;for(auto&g:profile.goals){if(!g.concept_id.valid()||!finite(g.priority)||!finite(g.commitment))return make_error(ErrorCode::InvalidArgument,"profile goal is invalid");if(!goal_ids.insert(g.concept_id).second)return make_error(ErrorCode::InvalidArgument,"profile contains duplicate goal concept");g.priority=clamp01(g.priority);g.commitment=clamp01(g.commitment);}
    profiles_.emplace(profile.id,std::move(profile));return {};
}
Result<void> ProfileRegistry::remove(ProfileId id){auto it=profiles_.find(id);if(it==profiles_.end())return make_error(ErrorCode::InvalidArgument,"profile does not exist");profiles_.erase(it);return {};}
void ProfileRegistry::clear()noexcept{profiles_.clear();}
bool ProfileRegistry::contains(ProfileId id)const noexcept{return profiles_.find(id)!=profiles_.end();}
const PsychologicalProfile* ProfileRegistry::find(ProfileId id)const noexcept{auto it=profiles_.find(id);return it==profiles_.end()?nullptr:&it->second;}
std::size_t ProfileRegistry::size()const noexcept{return profiles_.size();}
std::vector<ProfileId> ProfileRegistry::ids()const{std::vector<ProfileId> out;out.reserve(profiles_.size());for(const auto&kv:profiles_)out.push_back(kv.first);return out;}
Result<ResolvedProfile> ProfileRegistry::resolve(const ProfileRecipe& recipe,AgentId agent)const{
    if(recipe.components.empty())return make_error(ErrorCode::InvalidArgument,"profile recipe requires at least one component");
    if(!finite(recipe.variation_strength)||recipe.variation_strength<0.0f||recipe.variation_strength>1.0f)return make_error(ErrorCode::InvalidArgument,"profile variation strength must be in [0,1]");
    ResolvedProfile out{};out.personality.sociability=out.personality.stability=out.personality.empathy=out.personality.persistence=out.personality.curiosity=out.personality.boldness=0.0f;out.mood.valence=0.0f;out.mood.activation=0.0f;
    float total=0.0f;std::map<ConceptId,WeightedValue> values;std::map<ConceptId,WeightedGoal> goals;
    for(const auto&component:recipe.components){if(!component.profile.valid()||!finite(component.weight)||component.weight<=0.0f)return make_error(ErrorCode::InvalidArgument,"profile recipe contains invalid component");const auto*p=find(component.profile);if(!p)return make_error(ErrorCode::InvalidArgument,"profile recipe references unknown profile");const float w=component.weight;total+=w;out.personality.sociability+=p->personality.sociability*w;out.personality.stability+=p->personality.stability*w;out.personality.empathy+=p->personality.empathy*w;out.personality.persistence+=p->personality.persistence*w;out.personality.curiosity+=p->personality.curiosity*w;out.personality.boldness+=p->personality.boldness*w;out.mood.valence+=p->baseline_mood.valence*w;out.mood.activation+=p->baseline_mood.activation*w;for(const auto&v:p->values){auto&x=values[v.concept_id];x.sum+=v.intensity*w;x.weight+=w;}for(const auto&g:p->goals){auto&x=goals[g.concept_id];x.priority_sum+=g.priority*w;x.commitment_sum+=g.commitment*w;x.weight+=w;}}
    if(total<=0.0f||!finite(total))return make_error(ErrorCode::InvalidArgument,"profile recipe has invalid total weight");
    out.personality.sociability/=total;out.personality.stability/=total;out.personality.empathy/=total;out.personality.persistence/=total;out.personality.curiosity/=total;out.personality.boldness/=total;out.mood.valence=clamp11(out.mood.valence/total);out.mood.activation=clamp01(out.mood.activation/total);
    for(const auto&kv:values)out.values.push_back({kv.first,clamp01(kv.second.sum/kv.second.weight)});for(const auto&kv:goals)out.goals.push_back({kv.first,clamp01(kv.second.priority_sum/kv.second.weight),clamp01(kv.second.commitment_sum/kv.second.weight)});
    if(recipe.variation_strength>0.0f){std::uint64_t seed=recipe.variation_seed;if(agent.valid())seed=derive_agent_seed(seed,agent,0x50524f46494c45ULL);else if(seed==0)seed=0x4d53504150524f46ULL;DeterministicRng rng(seed);auto vary=[&](float&x){x=clamp01(x+(rng.next_unit_float()*2.0f-1.0f)*recipe.variation_strength);};vary(out.personality.sociability);vary(out.personality.stability);vary(out.personality.empathy);vary(out.personality.persistence);vary(out.personality.curiosity);vary(out.personality.boldness);}
    auto normalized=PersonalitySystem{}.normalize(out.personality);if(!normalized)return *normalized.error_if();return out;
}
}
