#pragma once
#include <map>
#include <memory>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/base/result.hpp"
#include "mspa/semantics/proposition.hpp"
namespace mspa {
struct CurvePointData { float x{0.0f}; float y{0.0f}; };
struct ConsiderationSchema { std::string id{}; float weight{1.0f}; std::vector<CurvePointData> curve{}; };
struct ActionSchema { ActionTypeId id{}; NamespaceId name_space{}; std::vector<std::string> considerations{}; };
struct PredicateSchema {
    PredicateId id{}; NamespaceId name_space{};
    std::vector<std::string> required_roles{}; std::vector<std::string> optional_roles{};
    std::string agency_role{}; std::vector<std::string> target_roles{};
    std::vector<std::string> consequence_tags{}; std::vector<std::string> appraisal_hints{};
    std::vector<ConceptId> related_concepts{};
    float default_relevance{0.5f}; float default_consequence{0.0f}; float intentionality{0.0f};
};
struct LoadedPackInfo { PackId id{}; std::uint32_t version_major{0}; std::uint32_t version_minor{0}; std::uint32_t version_patch{0}; };
struct InferenceRuleData {
    std::uint64_t id{0};
    std::vector<std::pair<PredicateId,bool>> premises{};
    PredicateId conclusion{}; NamespaceId conclusion_namespace{}; bool conclusion_positive{true}; float base_certainty{0.5f};
};
struct RegistryData {
    std::map<PredicateId,PredicateSchema> predicates{};
    std::map<ConceptId,std::string> concepts{};
    std::map<ActionTypeId,std::string> action_types{};
    std::map<ActionTypeId,ActionSchema> action_schemas{};
    std::map<std::string,ConsiderationSchema> considerations{};
    std::vector<InferenceRuleData> inference_rules{};
    std::vector<LoadedPackInfo> loaded_packs{};
    std::map<ConceptId,float> default_values{};
};
class RegistrySnapshot {
public:
    RegistrySnapshot(); explicit RegistrySnapshot(std::shared_ptr<const RegistryData> data);
    [[nodiscard]] const RegistryData& data() const noexcept; [[nodiscard]] const PredicateSchema* find(PredicateId id) const noexcept;
    [[nodiscard]] const ActionSchema* find_action(ActionTypeId id) const noexcept; [[nodiscard]] const ConsiderationSchema* find_consideration(const std::string& id) const noexcept;
private: std::shared_ptr<const RegistryData> data_;
};
class RegistryBuilder {
public:
    RegistryBuilder(); explicit RegistryBuilder(const RegistrySnapshot& base);
    Result<void> add_predicate(PredicateSchema schema); Result<void> add_concept(ConceptId id,std::string name); Result<void> add_action_type(ActionTypeId id,std::string name);
    Result<void> add_action_schema(ActionSchema schema); Result<void> add_consideration(ConsiderationSchema schema); Result<void> add_inference_rule(InferenceRuleData rule); Result<void> add_default_value(ConceptId id,float intensity); Result<void> mark_pack(PackId id,std::uint32_t major=0,std::uint32_t minor=0,std::uint32_t patch=0); [[nodiscard]] bool contains_pack(PackId id) const noexcept; [[nodiscard]] bool contains_pack(PackId id,std::uint32_t min_major,std::uint32_t min_minor=0) const noexcept; [[nodiscard]] const LoadedPackInfo* pack_info(PackId id) const noexcept; [[nodiscard]] const RegistryData& data() const noexcept;
    [[nodiscard]] RegistrySnapshot publish() const;
private: RegistryData data_{};
};
Result<void> validate_proposition(const Proposition& proposition,const RegistrySnapshot& registry,std::size_t max_args=64,std::size_t max_string_bytes=4096);
}
