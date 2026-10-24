#pragma once
#include <cstdint>
#include <string>
#include <utility>
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/base/version.hpp"
#include "mspa/semantics/inference.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
struct PackDependency { PackId id{}; std::uint32_t min_major{0}; std::uint32_t min_minor{0}; };
struct DomainPackMetadata { PackId id{}; NamespaceId name_space{}; std::uint32_t version_major{1},version_minor{0},version_patch{0}; std::uint32_t min_mspa_major{1}; std::string name{}; };
struct DomainPack {
    DomainPackMetadata metadata{}; std::vector<PackDependency> dependencies{};
    std::vector<PredicateSchema> predicates{}; std::vector<std::pair<ConceptId,std::string>> concepts{}; std::vector<std::pair<ActionTypeId,std::string>> action_types{};
    std::vector<ActionSchema> action_schemas{}; std::vector<ConsiderationSchema> considerations{}; std::vector<InferenceRule> inference_rules{};
    std::vector<std::string> context_tags{}; std::vector<std::string> outcome_tags{}; std::vector<std::pair<ConceptId,float>> initial_values{};
};
Result<void> validate_domain_pack(const DomainPack& pack); Result<void> apply_domain_pack(RegistryBuilder& builder,const DomainPack& pack);
}
