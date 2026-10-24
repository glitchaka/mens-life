#pragma once
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa { class ValueSystem { public: Result<void> set(AgentPsychState&,ConceptId,float)const; Result<void> erase(AgentPsychState&,ConceptId)const; [[nodiscard]] float intensity(const AgentPsychState&,ConceptId)const noexcept; }; }
