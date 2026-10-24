#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>
#include "mspa/base/result.hpp"
#include "mspa/semantics/registry.hpp"
namespace mspa {
struct InferencePremise { PredicateId predicate{}; bool positive{true}; };
struct InferenceRule { std::uint64_t id{0}; std::vector<PredicateId> premises{}; std::vector<InferencePremise> structured_premises{}; PredicateId conclusion{}; NamespaceId conclusion_namespace{}; bool conclusion_positive{true}; float base_certainty{0.5f}; };
struct InferenceBudget { std::size_t max_rules{256}; std::size_t max_hypotheses{64}; std::uint32_t max_depth{4}; };
struct Hypothesis { Proposition proposition{}; float certainty{0.0f}; std::uint32_t depth{1}; std::uint64_t rule_id{0}; };
class InferenceEngine {
public:
    Result<void> add_rule(InferenceRule rule); void clear_rules() noexcept; [[nodiscard]] std::size_t rule_count() const noexcept; [[nodiscard]] const std::vector<InferenceRule>& rules() const noexcept;
    Result<std::vector<Hypothesis>> infer(const std::vector<Proposition>& facts,const RegistrySnapshot& registry,const InferenceBudget& budget={}) const;
    static Result<std::vector<Hypothesis>> infer_registry_rules(const std::vector<Proposition>& facts,const RegistrySnapshot& registry,const InferenceBudget& budget={});
private:std::vector<InferenceRule> rules_{};
};
}
