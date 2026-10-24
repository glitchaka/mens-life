#pragma once
#include <cstddef>
#include <map>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {class AgentStore{public:Result<void>insert(AgentPsychState);Result<void>erase(AgentId);[[nodiscard]]bool contains(AgentId)const noexcept;[[nodiscard]]AgentPsychState*find(AgentId)noexcept;[[nodiscard]]const AgentPsychState*find(AgentId)const noexcept;[[nodiscard]]std::size_t size()const noexcept;[[nodiscard]]const std::map<AgentId,AgentPsychState>&all()const noexcept;private:std::map<AgentId,AgentPsychState>agents_{};};}
