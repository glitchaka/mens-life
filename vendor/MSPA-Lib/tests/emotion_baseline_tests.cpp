#include "test.hpp"
#include "mspa/mspa.hpp"
using namespace mspa;
int main(){
    RuntimeConfig cfg{};cfg.runtime_id=RuntimeId(1);cfg.world_id=WorldId(1);cfg.time_domain={TimeDomainId(1),{1,1}};cfg.seed_root=42;auto rr=Runtime::create(cfg);require(rr,"runtime");auto runtime=std::move(*rr.value_if());
    AgentId id{WorldId(1),1};AgentInit init{};init.personality={0.50f,0.65f,0.60f,0.65f,0.55f,0.50f};require(runtime->create_agent(id,init,PsychTime(0)),"create active agent");
    const auto* state=runtime->agents().find(id);require(state!=nullptr,"active agent state");require(state->emotions.size()==6,"active agent always has six base emotions");for(const auto&e:state->emotions)require(e.intensity>=active_emotion_floor,"active emotion never starts at zero");
    auto before=state->emotions;require(runtime->advance_to(id,PsychTime(1000000)),"advance");state=runtime->agents().find(id);require(state->emotions.size()==6,"decay does not delete emotions");for(const auto&e:state->emotions)require(e.intensity>=active_emotion_floor,"decay stops at tonic baseline");
    AgentPsychState legacy{};legacy.id={WorldId(1),2};legacy.personality=init.personality;legacy.last_processed_time=PsychTime(3);legacy.emotions.clear();AgentStore store;require(store.insert(legacy),"insert legacy active state");const auto* normalized=store.find(legacy.id);require(normalized&&normalized->emotions.size()==6,"legacy/imported active state gains all base emotions");for(const auto&e:normalized->emotions)require(e.intensity>=active_emotion_floor,"legacy state normalized above zero");
    require(!runtime->remove_agent(id,PsychTime(999999),RemovePolicy::ForgetRuntimeOnly),"cannot regress removal time");require(runtime->remove_agent(id,PsychTime(1000000),RemovePolicy::ForgetRuntimeOnly),"remove agent");require(!runtime->has_agent(id),"removed agent has no active emotional state");
    std::cout<<"emotion baseline ok\n";
}
