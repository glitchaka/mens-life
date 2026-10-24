#include "mspa/base/time.hpp"
#include <limits>
#include <numeric>
namespace mspa { namespace { bool mul(std::uint64_t a,std::uint64_t b,std::uint64_t& o){if(!a||!b){o=0;return true;} if(a>std::numeric_limits<std::uint64_t>::max()/b)return false;o=a*b;return true;} }
Result<PsychTime> convert_time_delta(PsychTime delta,TimeScale s,TimeScale d){if(!s.valid()||!d.valid())return make_error(ErrorCode::InvalidArgument,"invalid time scale"); std::uint64_t a=delta.ticks(),b=s.ticks_per_second_den,c=d.ticks_per_second_num,x=s.ticks_per_second_num,y=d.ticks_per_second_den; auto red=[](auto& p,auto& q){auto g=std::gcd(p,q);p/=g;q/=g;}; red(a,x);red(a,y);red(b,x);red(b,y);red(c,x);red(c,y);std::uint64_t t=0,n=0,den=0;if(!mul(a,b,t)||!mul(t,c,n)||!mul(x,y,den)||!den)return make_error(ErrorCode::InvalidState,"time conversion overflow"); return PsychTime(n/den);}
}
