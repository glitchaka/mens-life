#pragma once
#include "mspa/base/result.hpp"
#include "mspa/psychology/profile.hpp"
#include "mspa/runtime/runtime.hpp"
namespace mspa {
Result<AgentInit> materialize_profile_init(const ProfileRegistry& registry,const ProfileRecipe& recipe,AgentId agent,PsychTime now) noexcept;
Result<void> create_agent_from_profiles(Runtime& runtime,AgentId agent,const ProfileRegistry& registry,const ProfileRecipe& recipe,PsychTime now) noexcept;
}
