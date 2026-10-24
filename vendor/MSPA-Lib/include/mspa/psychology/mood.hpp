#pragma once
#include "mspa/model/agent_state.hpp"
namespace mspa { class MoodSystem { public: void update(AgentPsychState&,float,float,PsychTime)const noexcept; void absorb_emotions(AgentPsychState&,PsychTime,float gain=0.08f)const noexcept; void decay(AgentPsychState&,PsychTime,float rate_per_tick=0.00001f)const noexcept; void reset(AgentPsychState&,PsychTime)const noexcept; }; }
