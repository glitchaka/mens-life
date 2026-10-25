#pragma once
#include "mspa/model/agent_state.hpp"
#include "mspa/psychology/appraisal.hpp"
namespace mspa {
inline constexpr float active_emotion_floor=0.01f;
class EmotionSystem {
public:
    void ensure_baseline(AgentPsychState&,PsychTime)const;
    [[nodiscard]] float baseline(const AgentPsychState&,BasicEmotion)const noexcept;
    void apply(AgentPsychState&,const AppraisalVector&,PsychTime)const;
    void decay(AgentPsychState&,PsychTime,float base_rate_per_tick=0.0001f)const;
    [[nodiscard]] float intensity(const AgentPsychState&,BasicEmotion)const noexcept;
};
}
