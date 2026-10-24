#pragma once
#include <cstddef>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
#include "mspa/psychology/appraisal.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
using AppraisalResult = AppraisalVector;
struct ObservePsychologyConfig { std::size_t max_beliefs{4096}; std::size_t max_memories{4096}; std::size_t inference_max_rules{256}; std::size_t inference_max_hypotheses{64}; std::uint32_t inference_max_depth{4}; float perceived_certainty{0.75f}; float source_trust{0.5f}; BeliefSource source{BeliefSource::Observation}; AgentId source_agent{}; };
class PsychologyEngine {public:Result<bool>observe(AgentPsychState&,const Proposition&,PsychTime process_time,PsychTime event_time,bool has_event_time,const RegistrySnapshot&,AppraisalResult* appraisal_out=nullptr,const ObservePsychologyConfig& config = {})const;};
}
