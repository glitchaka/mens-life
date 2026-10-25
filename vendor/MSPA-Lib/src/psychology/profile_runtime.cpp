#include "mspa/psychology/profile_runtime.hpp"
#include <new>
namespace mspa {
Result<AgentInit> materialize_profile_init(const ProfileRegistry& registry,const ProfileRecipe& recipe,AgentId agent,PsychTime now) noexcept{
    try{
        auto resolved=registry.resolve(recipe,agent);if(!resolved)return *resolved.error_if();
        AgentInit init{};init.personality=resolved.value_if()->personality;init.values=resolved.value_if()->values;init.mood=resolved.value_if()->mood;init.mood.updated_at=now;
        std::uint64_t next_goal=1;for(const auto&seed:resolved.value_if()->goals){GoalState g{};g.id=GoalId(next_goal++);g.concept_id=seed.concept_id;g.priority=seed.priority;g.commitment=seed.commitment;g.status=GoalStatus::Active;g.created_at=now;g.updated_at=now;init.goals.push_back(g);}return init;
    }catch(const std::bad_alloc&){return make_error(ErrorCode::OutOfMemory,"profile materialization allocation failed");}catch(...){return make_error(ErrorCode::InternalInvariantViolation,"unexpected profile materialization exception");}
}
Result<void> create_agent_from_profiles(Runtime& runtime,AgentId agent,const ProfileRegistry& registry,const ProfileRecipe& recipe,PsychTime now) noexcept{
    auto init=materialize_profile_init(registry,recipe,agent,now);if(!init)return *init.error_if();return runtime.create_agent(agent,*init.value_if(),now);
}
}
