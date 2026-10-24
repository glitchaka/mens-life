#pragma once
#include <cstdint>
#include "mspa/base/ids.hpp"
namespace mspa {
struct RngState { std::uint64_t state{0}; };
enum class RngImportPolicy:std::uint8_t{RekeyForDestination,PreserveState};
std::uint64_t derive_agent_seed(std::uint64_t root,AgentId agent,std::uint64_t stream_kind) noexcept;
class DeterministicRng {
public:
    explicit DeterministicRng(std::uint64_t seed=1) noexcept; explicit DeterministicRng(RngState s) noexcept;
    [[nodiscard]] std::uint64_t next_u64() noexcept; [[nodiscard]] std::uint32_t next_u32() noexcept; [[nodiscard]] float next_unit_float() noexcept;
    [[nodiscard]] RngState state() const noexcept{return state_;}
private:RngState state_{};
};
} // namespace mspa
