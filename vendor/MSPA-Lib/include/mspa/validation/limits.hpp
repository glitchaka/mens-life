#pragma once
#include <cstddef>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
struct RuntimeLimits {
    std::size_t max_agents{100000}; std::size_t max_values_per_agent{1024}; std::size_t max_beliefs_per_agent{4096}; std::size_t max_emotions_per_agent{32}; std::size_t max_memories_per_agent{4096}; std::size_t max_goals_per_agent{256}; std::size_t max_relationships_per_agent{4096}; std::size_t max_learning_patterns_per_agent{4096}; std::size_t max_proposition_args{64}; std::size_t max_string_bytes{4096}; std::size_t max_serialized_bytes{64u*1024u*1024u}; std::size_t max_diagnostics{8192};
};
struct DefaultLimits {
    static constexpr std::size_t max_agents=100000,max_values_per_agent=1024,max_beliefs_per_agent=4096,max_emotions_per_agent=32,max_memories_per_agent=4096,max_goals_per_agent=256,max_relationships_per_agent=4096,max_learning_patterns_per_agent=4096,max_proposition_args=64,max_string_bytes=4096,max_serialized_bytes=64u*1024u*1024u,max_diagnostics=8192;
};
struct HardLimits {
    static constexpr std::size_t max_agents=1000000,max_values_per_agent=65536,max_beliefs_per_agent=65536,max_emotions_per_agent=256,max_memories_per_agent=65536,max_goals_per_agent=4096,max_relationships_per_agent=65536,max_learning_patterns_per_agent=65536,max_proposition_args=256,max_string_bytes=65536,max_serialized_bytes=512u*1024u*1024u,max_diagnostics=65536;
};
Result<void> validate_limits(const RuntimeLimits&); Result<void> validate_agent_state(const AgentPsychState&,const RuntimeLimits&); Result<void> validate_proposition_deep(const Proposition&,const RuntimeLimits&);
}
