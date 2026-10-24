#pragma once
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {class ReappraisalSystem{public:Result<void>reappraise_memory(AgentPsychState&,MemoryId,float new_emotional_load,float new_certainty)const;Result<void>reactivate_memory(AgentPsychState&,MemoryId)const;Result<void>reappraise_belief(AgentPsychState&,BeliefId,float emotional_load,float certainty,PsychTime now)const;};}
