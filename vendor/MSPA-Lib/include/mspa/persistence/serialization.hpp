#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/persistence/state_transfer.hpp"
namespace mspa {struct SerializationOptions{std::size_t max_bytes{64u*1024u*1024u};};Result<std::vector<std::uint8_t>>serialize_agent(const ExportedAgent&,const SerializationOptions& options = {});Result<ExportedAgent>deserialize_agent(const std::vector<std::uint8_t>&,const SerializationOptions& options = {});}
