#pragma once
#include <cstdint>
#include "mspa/base/ids.hpp"
#include "mspa/base/result.hpp"
namespace mspa {
class PsychTime {
public:
    constexpr PsychTime() noexcept=default; explicit constexpr PsychTime(std::uint64_t t) noexcept:ticks_(t){}
    [[nodiscard]] constexpr std::uint64_t ticks() const noexcept{return ticks_;}
    friend constexpr bool operator==(PsychTime a,PsychTime b) noexcept{return a.ticks_==b.ticks_;}
    friend constexpr bool operator!=(PsychTime a,PsychTime b) noexcept{return !(a==b);} friend constexpr bool operator<(PsychTime a,PsychTime b) noexcept{return a.ticks_<b.ticks_;}
    friend constexpr bool operator<=(PsychTime a,PsychTime b) noexcept{return a.ticks_<=b.ticks_;} friend constexpr bool operator>(PsychTime a,PsychTime b) noexcept{return b<a;} friend constexpr bool operator>=(PsychTime a,PsychTime b) noexcept{return b<=a;}
private: std::uint64_t ticks_{};
};
struct TimeScale { std::uint64_t ticks_per_second_num{1},ticks_per_second_den{1}; [[nodiscard]] constexpr bool valid()const noexcept{return ticks_per_second_num&&ticks_per_second_den;} };
struct TimeDomain { TimeDomainId id{}; TimeScale scale{}; [[nodiscard]] constexpr bool valid()const noexcept{return id.valid()&&scale.valid();} };
enum class ImportTimePolicy:std::uint8_t{RebaseRelative,PreserveAbsolute,RejectIncompatible};
struct StimulusTime { PsychTime process_time{}; bool has_event_time{false}; PsychTime event_time{}; };
Result<PsychTime> convert_time_delta(PsychTime delta, TimeScale source, TimeScale destination);
} // namespace mspa
