#pragma once
#include <string>
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {struct LearningReport{ActionTypeId action{};std::vector<std::string> context_tags{};std::vector<std::string> outcome_tags{};bool success{false};PsychTime now{};};class LearningSystem{public:void report(AgentPsychState&,ActionTypeId,bool)const;Result<void>report(AgentPsychState&,const LearningReport&,std::size_t max_patterns=4096)const;[[nodiscard]]float expected_success(const AgentPsychState&,ActionTypeId)const noexcept;[[nodiscard]]float expected_success(const AgentPsychState&,ActionTypeId,const std::vector<std::string>&context)const noexcept;[[nodiscard]]float expected_outcome(const AgentPsychState&,ActionTypeId,const std::vector<std::string>&context,const std::string&outcome)const noexcept;};}
