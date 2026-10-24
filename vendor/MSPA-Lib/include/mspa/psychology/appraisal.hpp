#pragma once
#include "mspa/model/agent_state.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
struct AppraisalVector { float relevance{0.0f}; float consequence{0.0f}; float agency{0.0f}; float intentionality{0.0f}; float certainty{0.5f}; float control{0.0f}; float norm_alignment{0.0f}; float expectation_error{0.0f}; float relationship{0.0f}; };
struct AppraisalContext { float perceived_certainty{0.75f}; float expected_consequence{0.0f}; float perceived_control{-1.0f}; };
class AppraisalEngine { public: [[nodiscard]] AppraisalVector evaluate(const AgentPsychState&,const Proposition&,const RegistrySnapshot&,const AppraisalContext& context = {})const; };
}
