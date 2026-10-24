#pragma once
#include "mspa/decision/decision.hpp"
namespace mspa {class HysteresisPolicy{public:[[nodiscard]]float continuity_bonus(const AgentPsychState&,const ActionOption&,ActionContinuityKey current,float urgency=0.0f)const noexcept;[[nodiscard]]float switch_threshold(const AgentPsychState&,const ActionOption&current,float urgency=0.0f)const noexcept;};}
