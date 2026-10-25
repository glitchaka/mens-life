#pragma once
#include <cstddef>
#include <cstdint>
#include <map>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {
struct ProfileGoalSeed{ConceptId concept_id{};float priority{0.5f};float commitment{0.5f};};
struct PsychologicalProfile{
    ProfileId id{};
    std::string name{};
    std::string description{};
    Personality personality{};
    std::vector<ValueEntry> values{};
    MoodState baseline_mood{};
    std::vector<ProfileGoalSeed> goals{};
};
struct ProfileWeight{ProfileId profile{};float weight{1.0f};};
struct ProfileRecipe{
    std::vector<ProfileWeight> components{};
    std::uint64_t variation_seed{0};
    float variation_strength{0.0f};
};
struct ResolvedProfile{
    Personality personality{};
    std::vector<ValueEntry> values{};
    MoodState mood{};
    std::vector<ProfileGoalSeed> goals{};
};
class ProfileRegistry{
public:
    Result<void> add(PsychologicalProfile profile);
    Result<void> remove(ProfileId id);
    void clear() noexcept;
    [[nodiscard]] bool contains(ProfileId id)const noexcept;
    [[nodiscard]] const PsychologicalProfile* find(ProfileId id)const noexcept;
    [[nodiscard]] std::size_t size()const noexcept;
    [[nodiscard]] std::vector<ProfileId> ids()const;
    Result<ResolvedProfile> resolve(const ProfileRecipe& recipe,AgentId agent={})const;
private:
    std::map<ProfileId,PsychologicalProfile> profiles_{};
};
}
