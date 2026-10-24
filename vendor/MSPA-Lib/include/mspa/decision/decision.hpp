#pragma once
#include <map>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/model/agent_state.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
enum class EffectChannel:std::uint8_t{Need,Value,Goal,Relationship,Emotion,Generic};
struct NeedSignal{ConceptId concept_id{};float pressure{0}; };
struct DecisionContext{std::vector<NeedSignal>needs{};std::map<std::string,float>signals{};float urgency{0.0f};};
struct ExpectedEffect{EffectChannel channel{EffectChannel::Generic};ConceptId concept_id{};AgentId target{};float magnitude{0};std::string outcome_tag{};};
struct ActionOption{ActionTypeId type{};ActionInstanceId instance{};ActionContinuityKey continuity{};AgentId actor{};std::vector<AgentId>targets{};float cost{0},risk{0},progress{0},novelty{0},social_exposure{0},interruption_cost{0},context_stability{1};std::vector<std::string>context_tags{};std::vector<ExpectedEffect>expected_effects{};std::map<std::string,float>signals{};};
struct ConsiderationScore{std::string name{};float raw{0},weighted{0};};
struct CandidateScore{ActionInstanceId action{};float utility{0};std::vector<ConsiderationScore>considerations{};};
struct Decision{bool has_selection{false};ActionInstanceId selected_action{};AgentId selected_target{};float confidence{0};std::string intention{};std::string expressed_emotion{};std::vector<std::string>outgoing_statements{};std::vector<CandidateScore>trace{};};
class UtilityComposer{public:[[nodiscard]]CandidateScore score(const ActionOption&)const;[[nodiscard]]CandidateScore score(const ActionOption&,const AgentPsychState&,const DecisionContext& context = {})const;[[nodiscard]]CandidateScore score(const ActionOption&,const AgentPsychState&,const DecisionContext&,const RegistrySnapshot&)const;};
}
