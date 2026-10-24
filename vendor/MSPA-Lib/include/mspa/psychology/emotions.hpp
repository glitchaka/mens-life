#pragma once
#include "mspa/model/agent_state.hpp"
#include "mspa/psychology/appraisal.hpp"
namespace mspa { class EmotionSystem { public: void apply(AgentPsychState&,const AppraisalVector&,PsychTime)const; void decay(AgentPsychState&,PsychTime,float base_rate_per_tick=0.0001f)const; [[nodiscard]] float intensity(const AgentPsychState&,BasicEmotion)const noexcept; }; }
