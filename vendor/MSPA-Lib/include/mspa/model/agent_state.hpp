#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/base/rng.hpp"
#include "mspa/base/time.hpp"
#include "mspa/base/version.hpp"
#include "mspa/semantics/proposition.hpp"
namespace mspa {
struct Personality { float sociability{0.5f},stability{0.5f},empathy{0.5f},persistence{0.5f},curiosity{0.5f},boldness{0.5f}; };
struct ValueEntry { ConceptId concept_id{}; float intensity{0.0f}; };
enum class BeliefSource:std::uint8_t{Observation,Testimony,Inference,Memory,Influence,Learning,Unknown};
struct BeliefState { BeliefId id{}; Proposition proposition{}; float certainty{0.0f}; float emotional_load{0.0f}; BeliefSource source{BeliefSource::Unknown}; AgentId source_agent{}; std::uint32_t evidence_count{1}; PsychTime updated_at{}; };
enum class BasicEmotion:std::uint8_t{Joy,Sadness,Fear,Anger,Anxiety,Aversion};
struct EmotionState { BasicEmotion kind{BasicEmotion::Joy}; float intensity{0.0f}; PsychTime started_at{}; PsychTime updated_at{}; };
struct MoodState { float valence{0.0f}; float activation{0.0f}; PsychTime updated_at{}; };
enum class GoalStatus:std::uint8_t{Active,Suspended,Blocked,Completed,Abandoned};
struct GoalState { GoalId id{}; ConceptId concept_id{}; float priority{0.0f}; float commitment{0.0f}; GoalStatus status{GoalStatus::Active}; PsychTime created_at{}; PsychTime updated_at{}; [[nodiscard]] bool active()const noexcept{return status==GoalStatus::Active;} };
struct MemoryState { MemoryId id{}; Proposition proposition{}; float importance{0.0f}; float emotional_load{0.0f}; float certainty{1.0f}; BeliefSource source{BeliefSource::Memory}; std::vector<AgentId> participants{}; std::uint32_t reactivations{0}; PsychTime event_time{}; PsychTime last_reactivated_at{}; };
struct RelationshipState { AgentId other{}; float affection{0.0f},trust{0.0f},respect{0.0f},familiarity{0.0f}; };
struct LearnedExpectation { ActionTypeId action{}; std::vector<std::string> context_tags{}; std::string outcome_tag{"success"}; std::uint32_t attempts{0},successes{0}; float confidence{0.5f}; PsychTime last_activation{}; };
struct AgentPsychState {
    std::uint32_t schema_version{state_schema_version}; AgentId id{}; Personality personality{}; std::vector<ValueEntry> values{}; std::vector<BeliefState> beliefs{}; std::vector<EmotionState> emotions{}; MoodState mood{}; std::vector<GoalState> goals{}; std::vector<MemoryState> memories{}; std::vector<RelationshipState> relationships{}; std::vector<LearnedExpectation> learned_expectations{}; ActionContinuityKey current_intent{}; RngState rng_state{}; PsychTime last_processed_time{};
};
}
