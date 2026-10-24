#pragma once
#include <cstddef>
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa { class MemorySystem {public:Result<MemoryId>remember(AgentPsychState&,const Proposition&,float,PsychTime,float emotional_load=0.0f,float certainty=1.0f,BeliefSource source=BeliefSource::Memory,const std::vector<AgentId>& participants={})const;Result<MemoryId>remember(AgentPsychState&,PredicateId,float,PsychTime)const;Result<void>reactivate(AgentPsychState&,MemoryId,PsychTime)const;[[nodiscard]]std::vector<const MemoryState*>recall(const AgentPsychState&,PredicateId,std::size_t limit=8)const;void compact(AgentPsychState&,std::size_t max_memories)const;void consolidate(AgentPsychState&,PsychTime,std::size_t max_memories,float age_penalty_per_tick=0.000001f)const;}; }
