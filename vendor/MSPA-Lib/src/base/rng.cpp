#include "mspa/base/rng.hpp"
namespace mspa {
namespace { std::uint64_t mix(std::uint64_t x) noexcept { x+=0x9e3779b97f4a7c15ULL; x=(x^(x>>30))*0xbf58476d1ce4e5b9ULL; x=(x^(x>>27))*0x94d049bb133111ebULL; return x^(x>>31); } }
std::uint64_t derive_agent_seed(std::uint64_t root,AgentId a,std::uint64_t stream) noexcept { return mix(root ^ mix(a.world.value()) ^ mix(a.local_id) ^ mix(stream)); }
DeterministicRng::DeterministicRng(std::uint64_t seed) noexcept { state_.state=seed?seed:0x853c49e6748fea9bULL; }
DeterministicRng::DeterministicRng(RngState s) noexcept { state_.state=s.state?s.state:0x853c49e6748fea9bULL; }
std::uint64_t DeterministicRng::next_u64() noexcept { std::uint64_t x=state_.state; x^=x>>12; x^=x<<25; x^=x>>27; state_.state=x; return x*2685821657736338717ULL; }
std::uint32_t DeterministicRng::next_u32() noexcept { return static_cast<std::uint32_t>(next_u64()>>32); }
float DeterministicRng::next_unit_float() noexcept { return static_cast<float>(next_u32())/4294967295.0f; }
} // namespace mspa
