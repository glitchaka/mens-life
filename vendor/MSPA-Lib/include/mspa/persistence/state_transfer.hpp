#pragma once
#include <cstdint>
#include "mspa/base/result.hpp"
#include "mspa/base/rng.hpp"
#include "mspa/base/time.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {struct ExportedAgent{AgentPsychState state{};TimeDomain origin_time_domain{};PsychTime exported_at{};};struct ImportOptions{AgentId destination_id{};PsychTime import_time{};ImportTimePolicy time_policy{ImportTimePolicy::RebaseRelative};RngImportPolicy rng_policy{RngImportPolicy::RekeyForDestination};std::uint64_t destination_seed_root{0};};struct ImportResult{AgentPsychState state{};AgentId original_id{};AgentId effective_id{};ImportTimePolicy time_policy{ImportTimePolicy::RebaseRelative};RngImportPolicy rng_policy{RngImportPolicy::RekeyForDestination};};Result<ImportResult>import_agent_state(const ExportedAgent&,const TimeDomain&,const ImportOptions&);Result<void>migrate_agent_state(AgentPsychState&,std::uint32_t target_version=state_schema_version);}
