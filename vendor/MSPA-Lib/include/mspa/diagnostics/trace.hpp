#pragma once
#include <cstddef>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/base/time.hpp"
namespace mspa {struct TraceItem{std::string stage{},detail{};float weight{0};};struct DiagnosticTrace{std::uint64_t token{0};AgentId agent{};PsychTime process_time{};bool has_event_time{false};PsychTime event_time{};std::vector<TraceItem>items{};};class TraceSink{public:explicit TraceSink(std::size_t max=8192):max_(max){}std::uint64_t push(DiagnosticTrace);[[nodiscard]]const std::vector<DiagnosticTrace>&traces()const noexcept;[[nodiscard]]const DiagnosticTrace*find(std::uint64_t token)const noexcept;void clear()noexcept;void set_max(std::size_t max)noexcept;private:std::vector<DiagnosticTrace>traces_{};std::size_t max_{8192};std::uint64_t next_{1};};}
