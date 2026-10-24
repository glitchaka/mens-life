#pragma once
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {
struct BeliefEvidence { Proposition proposition{}; float strength{0.5f}; float source_trust{0.5f}; BeliefSource source{BeliefSource::Observation}; AgentId source_agent{}; float emotional_load{0.0f}; PsychTime observed_at{}; };
struct BeliefUpdate { BeliefId id{}; bool created{false}; bool flipped{false}; float old_certainty{0.0f}; float new_certainty{0.0f}; };
class BeliefSystem {
public:
    Result<BeliefId> add(AgentPsychState&,const Proposition&,float,float,PsychTime,BeliefSource=BeliefSource::Unknown,AgentId={})const;
    Result<BeliefId> add(AgentPsychState&,PredicateId,float,float,PsychTime)const;
    Result<void> reinforce(AgentPsychState&,BeliefId,float)const;
    Result<BeliefUpdate> update(AgentPsychState&,const BeliefEvidence&)const;
    [[nodiscard]] const BeliefState* find(const AgentPsychState&,const Proposition&)const noexcept;
};
}
