#pragma once
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa { struct GoalSignal{ConceptId concept_id{};float strength{0.0f};float commitment{0.0f};}; class GoalSystem {public:Result<GoalId> add(AgentPsychState&,ConceptId,float)const;Result<GoalId> add(AgentPsychState&,ConceptId,float,float,PsychTime)const;Result<void>set_status(AgentPsychState&,GoalId,GoalStatus,PsychTime)const;Result<void>complete(AgentPsychState&,GoalId)const;Result<void>derive(AgentPsychState&,const std::vector<GoalSignal>&,PsychTime,std::size_t max_active=3)const;void decay(AgentPsychState&,PsychTime,float rate_per_tick=0.00001f)const noexcept;}; }
