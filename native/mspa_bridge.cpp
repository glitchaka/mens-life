#include "mspa_bridge.h"
#include <mspa/mspa.hpp>
#include <algorithm>
#include <array>
#include <cmath>
#include <cstring>
#include <exception>
#include <memory>
#include <stdexcept>
#include <string>

namespace {
constexpr mspa::NamespaceId domain{1};
constexpr mspa::WorldId world{1};
constexpr const char* names[] = {"energy", "food", "hygiene", "fun", "bladder"};

template <typename T> void require(const mspa::Result<T>& result) {
    if (!result) throw std::runtime_error(result.error_if()->details);
}

void copy_text(const char* text, char* output, size_t capacity) noexcept {
    if (!output || capacity == 0) return;
    const auto length = std::min(std::strlen(text), capacity - 1);
    std::memcpy(output, text, length);
    output[length] = '\0';
}

mspa::Personality personality_for(uint64_t profile_id) {
    mspa::Personality p;
    const std::array<std::array<float, 6>, 6> profiles{{
        {{0.70f,0.72f,0.80f,0.58f,0.74f,0.54f}},
        {{0.86f,0.54f,0.64f,0.46f,0.86f,0.82f}},
        {{0.78f,0.38f,0.34f,0.64f,0.58f,0.90f}},
        {{0.56f,0.84f,0.84f,0.74f,0.54f,0.40f}},
        {{0.66f,0.62f,0.56f,0.66f,0.78f,0.62f}},
        {{0.86f,0.56f,0.88f,0.48f,0.72f,0.74f}},
    }};
    const auto& v = profiles[std::clamp<uint64_t>(profile_id, 1, profiles.size()) - 1];
    p.sociability=v[0]; p.stability=v[1]; p.empathy=v[2];
    p.persistence=v[3]; p.curiosity=v[4]; p.boldness=v[5];
    return p;
}

mspa::DomainPack house_pack() {
    mspa::DomainPack pack;
    pack.metadata.id = mspa::PackId{1};
    pack.metadata.name_space = domain;
    pack.metadata.name = "mens-life.house";
    pack.context_tags = {"house", "care", "leisure", "interrupted", "environment", "social"};
    pack.outcome_tags = {"success", "blocked", "need_restored", "noticed", "positive_contact", "interrupted_contact"};
    for (uint64_t i = 0; i < 5; ++i) {
        const auto concept = mspa::ConceptId{i + 1};
        pack.concepts.push_back({concept, names[i]});
        pack.initial_values.push_back({concept, 0.45f});
        pack.action_types.push_back({mspa::ActionTypeId{i + 1}, names[i]});
        for (uint64_t kind = 1; kind <= 6; ++kind) {
            mspa::PredicateSchema schema;
            schema.id = mspa::PredicateId{kind * 100 + i};
            schema.name_space = domain;
            schema.required_roles = {"subject"};
            schema.related_concepts = {concept};
            const float consequences[] = {-0.35f, -0.85f, 0.06f, 0.65f, -0.65f, -0.08f};
            schema.default_consequence = consequences[kind - 1];
            schema.default_relevance = kind == 2 ? 0.95f : kind == 3 ? 0.2f : 0.65f;
            pack.predicates.push_back(std::move(schema));
        }
    }
    pack.action_types.push_back({mspa::ActionTypeId{6}, "rest"});
    pack.concepts.push_back({mspa::ConceptId{6}, "connection"});
    pack.initial_values.push_back({mspa::ConceptId{6}, 0.7f});
    for (uint64_t i = 0; i < 3; ++i)
        pack.action_types.push_back({mspa::ActionTypeId{7+i}, i == 0 ? "greet" : i == 1 ? "chat" : "joke"});
    for (uint64_t i = 0; i < 4; ++i) {
        mspa::PredicateSchema schema;
        schema.id = mspa::PredicateId{1000+i};
        schema.name_space = domain;
        schema.required_roles = {"subject", "other"};
        schema.agency_role = "other";
        schema.target_roles = {"other"};
        schema.related_concepts = {mspa::ConceptId{6}};
        schema.default_relevance = 0.65f;
        schema.default_consequence = i == 3 ? -0.3f : 0.5f;
        schema.intentionality = 0.65f;
        pack.predicates.push_back(std::move(schema));
    }
    const float environmental_consequence[] = {0.03f, 0.14f, 0.05f, 0.10f, -0.03f};
    for (uint64_t i = 0; i < 5; ++i) {
        mspa::PredicateSchema schema;
        schema.id = mspa::PredicateId{1200+i};
        schema.name_space = domain;
        schema.required_roles = {"subject"};
        schema.related_concepts = {mspa::ConceptId{i+1}};
        schema.default_relevance = 0.32f;
        schema.default_consequence = environmental_consequence[i];
        schema.intentionality = 0.0f;
        pack.predicates.push_back(std::move(schema));
    }
    return pack;
}
}

struct ml_brain {
    std::unique_ptr<mspa::Runtime> runtime;
    mspa::AgentId character{world, 1};
    uint64_t last_social{0};
    std::array<float, 5> values{74, 61, 82, 55, 70};
    std::array<int, 5> bands{-1, -1, -1, -1, -1};
    mspa::PsychTime now{};
    uint64_t sequence{0};
    float confidence{0};
    std::array<char, 512> error{};

    mspa::Proposition proposition(int task, int kind, bool positive = true) const {
        mspa::Proposition p;
        p.predicate = mspa::PredicateId{uint64_t(kind * 100 + task)};
        p.name_space = domain;
        p.positive = positive;
        p.provenance = "mens-life:physical-simulation";
        mspa::PropositionArg subject;
        subject.role = "subject";
        subject.has_agent = true;
        subject.agent = character;
        p.arguments.push_back(std::move(subject));
        return p;
    }
    void observe(int task, int kind, bool positive = true) {
        mspa::ObserveInput input;
        input.agent = character;
        input.proposition = proposition(task, kind, positive);
        input.time.process_time = now;
        input.perceived_certainty = 1;
        input.source_trust = 1;
        require(runtime->observe(input));
    }
    template <typename F> int32_t guarded(F operation) noexcept {
        try { operation(); error[0] = '\0'; return 1; }
        catch (const std::exception& exception) { copy_text(exception.what(), error.data(), error.size()); }
        catch (...) { copy_text("Unknown MSPA bridge error", error.data(), error.size()); }
        return 0;
    }
};

extern "C" ml_brain* ml_brain_create_profile(uint64_t seed, uint64_t agent_id, uint64_t profile_id,
                                                char* error, size_t capacity) noexcept {
    try {
        auto brain = std::make_unique<ml_brain>();
        if (!agent_id) throw std::runtime_error("Invalid character identity");
        if (profile_id < 1 || profile_id > 6) throw std::runtime_error("Invalid personality profile");
        brain->character = {world, agent_id};
        mspa::RegistryBuilder builder;
        require(mspa::apply_domain_pack(builder, house_pack()));
        mspa::RuntimeConfig config;
        config.runtime_id = mspa::RuntimeId{agent_id};
        config.world_id = world;
        config.time_domain = {mspa::TimeDomainId{1}, {1000, 1}};
        config.seed_root = seed;
        config.limits.max_agents = 1;
        config.limits.max_memories_per_agent = 256;
        config.limits.max_beliefs_per_agent = 128;
        config.limits.max_learning_patterns_per_agent = 64;
        config.limits.max_diagnostics = 128;
        auto created = mspa::Runtime::create(config, builder.publish());
        require(created);
        brain->runtime = std::move(*created.value_if());
        mspa::AgentInit init;
        init.personality = personality_for(profile_id);
        for (uint64_t other = 1; other <= 6; ++other) {
            if (other != agent_id) init.relationships.push_back({{world,other},0.0f,0.1f,0.1f,0.0f});
        }
        init.use_registry_default_values = true;
        for (uint64_t i = 0; i < 5; ++i) {
            mspa::GoalState goal;
            goal.id = mspa::GoalId{i + 1};
            goal.concept_id = mspa::ConceptId{i + 1};
            goal.priority = 0.25f;
            goal.commitment = 0.6f;
            init.goals.push_back(goal);
        }
        require(brain->runtime->create_agent(brain->character, init, brain->now));
        copy_text("", error, capacity);
        return brain.release();
    } catch (const std::exception& exception) { copy_text(exception.what(), error, capacity); }
      catch (...) { copy_text("Cannot create MSPA runtime", error, capacity); }
    return nullptr;
}

extern "C" ml_brain* ml_brain_create_agent(uint64_t seed, uint64_t agent_id, char* error, size_t capacity) noexcept {
    return ml_brain_create_profile(seed, agent_id, std::clamp<uint64_t>(agent_id, 1, 6), error, capacity);
}
extern "C" ml_brain* ml_brain_create(uint64_t seed, char* error, size_t capacity) noexcept {
    return ml_brain_create_profile(seed, 1, 1, error, capacity);
}
extern "C" void ml_brain_destroy(ml_brain* brain) noexcept { delete brain; }

extern "C" int32_t ml_brain_update(ml_brain* brain, uint64_t ticks, const float values[5]) noexcept {
    if (!brain || !values) return 0;
    return brain->guarded([&] {
        const mspa::PsychTime next{ticks};
        require(brain->runtime->advance_to(brain->character, next));
        brain->now = next;
        for (int i = 0; i < 5; ++i) {
            if (!std::isfinite(values[i])) throw std::runtime_error("Non-finite physical need");
            brain->values[i] = std::clamp(values[i], 0.0f, 100.0f);
            const int band = values[i] <= 15 ? 2 : values[i] <= 35 ? 1 : 0;
            const int previous = brain->bands[i];
            if (band == previous) continue;
            if ((previous >= 1) != (band >= 1)) brain->observe(i, 1, band >= 1);
            if ((previous == 2) != (band == 2)) brain->observe(i, 2, band == 2);
            brain->bands[i] = band;
        }
    });
}

extern "C" int32_t ml_brain_decide(ml_brain* brain, const ml_option* offered, size_t count,
                                    int32_t current, int32_t* selected) noexcept {
    if (!brain || !selected || !offered || count == 0 || count > 6) return 0;
    return brain->guarded([&] {
        mspa::DecisionContext context;
        float highest = 0;
        for (uint64_t i = 0; i < 5; ++i) {
            const float pressure = 1.0f - brain->values[i] / 100.0f;
            context.needs.push_back({mspa::ConceptId{i + 1}, pressure * pressure});
            highest = std::max(highest, pressure);
        }
        context.urgency = std::clamp((highest - 0.65f) / 0.35f, 0.0f, 1.0f);
        std::vector<mspa::ActionOption> options;
        const auto base = ++brain->sequence * 8;
        for (size_t i = 0; i < count; ++i) {
            if (offered[i].task < -1 || offered[i].task > 4) throw std::runtime_error("Invalid offered activity");
            const uint64_t key = offered[i].task < 0 ? 6 : uint64_t(offered[i].task + 1);
            mspa::ActionOption option;
            option.type = mspa::ActionTypeId{key};
            option.instance = mspa::ActionInstanceId{base + i + 1};
            option.continuity = mspa::ActionContinuityKey{key};
            option.actor = brain->character;
            option.cost = offered[i].cost;
            option.progress = offered[i].progress;
            option.interruption_cost = offered[i].task == current ? 0.35f : 0;
            option.context_tags = {"house", key == 4 || key == 6 ? "leisure" : "care"};
            if (key != 6) {
                const auto concept = mspa::ConceptId{key};
                option.expected_effects.push_back({mspa::EffectChannel::Need, concept, {}, 1.4f, "need_restored"});
                option.expected_effects.push_back({mspa::EffectChannel::Value, concept, {}, 0.08f, {}});
                option.expected_effects.push_back({mspa::EffectChannel::Goal, concept, {}, 0.08f, {}});
            } else option.expected_effects.push_back({mspa::EffectChannel::Generic, {}, {}, 0.22f, {}});
            options.push_back(std::move(option));
        }
        const auto continuity = current >= 0 ? mspa::ActionContinuityKey{uint64_t(current + 1)} : mspa::ActionContinuityKey{};
        auto decision = brain->runtime->decide(brain->character, options, context, brain->now, continuity);
        require(decision);
        *selected = -1;
        if (decision.value_if()->has_selection) {
            const auto index = decision.value_if()->selected_action.value() - base - 1;
            if (index >= count) throw std::runtime_error("MSPA selected an unavailable activity");
            *selected = offered[index].task;
        }
        brain->confidence = decision.value_if()->confidence;
    });
}

extern "C" int32_t ml_brain_event(ml_brain* brain, int32_t task, int32_t kind) noexcept {
    if (!brain || task < 0 || task >= 5 || kind < 0 || kind > 3) return 0;
    return brain->guarded([&] {
        if (kind == 0) brain->observe(task, 3);
        else if (kind == 3) brain->observe(task, 6);
        else {
            mspa::ReportInput report;
            report.agent = brain->character;
            report.action_type = mspa::ActionTypeId{uint64_t(task + 1)};
            report.process_time = brain->now;
            report.success = kind == 1;
            report.context_tags = {"house", task == 3 ? "leisure" : "care"};
            report.outcome_tags = kind == 1 ? std::vector<std::string>{"success", "need_restored"} : std::vector<std::string>{"blocked"};
            report.resulting_propositions.push_back(brain->proposition(task, kind == 1 ? 4 : 5));
            require(brain->runtime->report(report));
        }
        if (kind != 0) require(brain->runtime->decide(brain->character, {}, brain->now));
    });
}

extern "C" int32_t ml_brain_environment(ml_brain* brain, int32_t kind, float salience) noexcept {
    if (!brain || kind < 0 || kind >= 5 || !std::isfinite(salience)) return 0;
    return brain->guarded([&] {
        mspa::Proposition p;
        p.predicate = mspa::PredicateId{uint64_t(1200 + kind)};
        p.name_space = domain;
        p.positive = true;
        p.provenance = "mens-life:environment-perception";
        mspa::PropositionArg subject;
        subject.role = "subject";
        subject.has_agent = true;
        subject.agent = brain->character;
        p.arguments.push_back(std::move(subject));
        mspa::ObserveInput input;
        input.agent = brain->character;
        input.proposition = std::move(p);
        input.time.process_time = brain->now;
        input.perceived_certainty = std::clamp(salience, 0.0f, 1.0f);
        input.source_trust = 0.75f;
        require(brain->runtime->observe(input));
    });
}

extern "C" int32_t ml_brain_state(const ml_brain* brain, ml_psych_state* output) noexcept {
    if (!brain || !output) return 0;
    const auto* state = brain->runtime->agents().find(brain->character);
    if (!state) return 0;
    *output = {};
    for (const auto& emotion : state->emotions) output->emotions[static_cast<size_t>(emotion.kind)] = emotion.intensity;
    output->valence = state->mood.valence;
    output->activation = state->mood.activation;
    output->confidence = brain->confidence;
    output->memories = static_cast<uint32_t>(state->memories.size());
    output->beliefs = static_cast<uint32_t>(state->beliefs.size());
    output->learned_patterns = static_cast<uint32_t>(state->learned_expectations.size());
    output->personality[0] = state->personality.sociability;
    output->personality[1] = state->personality.stability;
    output->personality[2] = state->personality.empathy;
    output->personality[3] = state->personality.persistence;
    output->personality[4] = state->personality.curiosity;
    output->personality[5] = state->personality.boldness;
    const auto derived = mspa::PersonalitySystem{}.derive(state->personality);
    output->derived_traits[0] = derived.belief_rigidity;
    output->derived_traits[1] = derived.goal_persistence;
    output->derived_traits[2] = derived.switch_resistance;
    output->derived_traits[3] = derived.threat_sensitivity;
    output->derived_traits[4] = derived.social_sensitivity;
    output->derived_traits[5] = derived.exploration;
    return 1;
}

extern "C" void ml_brain_error(const ml_brain* brain, char* output, size_t capacity) noexcept {
    copy_text(brain ? brain->error.data() : "Missing MSPA runtime", output, capacity);
}

extern "C" int32_t ml_brain_social_decide(ml_brain* brain, uint64_t target, int32_t* selected) noexcept {
    if (!brain || !selected || !target || target == brain->character.local_id) return 0;
    return brain->guarded([&] {
        const auto* state = brain->runtime->agents().find(brain->character);
        const float urge = std::clamp(float(brain->now.ticks()-brain->last_social)/18000.0f,0.0f,1.0f);
        float pressure = 0;
        for (float value : brain->values) pressure = std::max(pressure,1.0f-value/100.0f);
        std::vector<mspa::ActionOption> options;
        const auto base = ++brain->sequence*8;
        for (uint64_t kind = 0; kind < 3; ++kind) {
            mspa::ActionOption option;
            option.type = mspa::ActionTypeId{7+kind};
            option.instance = mspa::ActionInstanceId{base+kind+1};
            option.actor = brain->character;
            option.targets = {{world,target}};
            option.cost = 0.07f+pressure*0.3f;
            option.risk = kind == 2 ? 0.12f : 0.01f;
            option.novelty = kind == 2 ? 0.5f : 0.2f;
            option.social_exposure = 0.3f;
            option.context_tags = {"house","social"};
            option.expected_effects.push_back({mspa::EffectChannel::Value,mspa::ConceptId{6},{},0.3f,{}});
            option.expected_effects.push_back({mspa::EffectChannel::Relationship,{}, {world,target},0.5f,{}});
            option.expected_effects.push_back({mspa::EffectChannel::Generic,{}, {},urge*state->personality.sociability*0.5f,{}});
            options.push_back(std::move(option));
        }
        mspa::ActionOption rest;
        rest.type = mspa::ActionTypeId{6}; rest.instance = mspa::ActionInstanceId{base+4}; rest.actor = brain->character;
        rest.expected_effects.push_back({mspa::EffectChannel::Generic,{}, {},0.2f,{}});
        options.push_back(rest);
        auto decision = brain->runtime->decide(brain->character,options,brain->now);
        require(decision);
        const auto index = decision.value_if()->selected_action.value()-base-1;
        *selected = decision.value_if()->has_selection && index < 3 ? int32_t(index) : -1;
    });
}

extern "C" int32_t ml_brain_social_report(ml_brain* brain, uint64_t target, int32_t kind, int32_t success) noexcept {
    if (!brain || !target || kind < 0 || kind > 2) return 0;
    return brain->guarded([&] {
        mspa::ReportInput report;
        report.agent = brain->character;
        report.action_type = mspa::ActionTypeId{uint64_t(7+kind)};
        report.success = success != 0;
        report.process_time = brain->now;
        report.context_tags = {"house","social"};
        report.outcome_tags = {success ? "positive_contact" : "interrupted_contact"};
        mspa::Proposition p;
        p.predicate = mspa::PredicateId{uint64_t(success ? 1000+kind : 1003)};
        p.name_space = domain;
        p.provenance = "mens-life:completed-social-interaction";
        mspa::PropositionArg subject; subject.role = "subject"; subject.has_agent = true; subject.agent = brain->character;
        mspa::PropositionArg other; other.role = "other"; other.has_agent = true; other.agent = {world,target};
        p.arguments = {subject,other}; report.resulting_propositions.push_back(p);
        mspa::RelationshipImpact impact;
        impact.target = {world,target};
        impact.delta = success ? mspa::RelationshipDelta{kind == 2 ? 0.06f : 0.04f,0.025f,0.015f,0.08f}
                               : mspa::RelationshipDelta{-0.01f,0.0f,0.0f,0.01f};
        report.relationship_impacts.push_back(impact);
        require(brain->runtime->report(report));
        brain->last_social = brain->now.ticks();
        require(brain->runtime->decide(brain->character,{},brain->now));
    });
}

extern "C" int32_t ml_brain_relation(const ml_brain* brain, uint64_t target, float output[4]) noexcept {
    if (!brain || !target || !output) return 0;
    const auto* state = brain->runtime->agents().find(brain->character);
    const auto* relation = mspa::RelationshipSystem{}.find(*state,{world,target});
    output[0] = relation ? relation->affection : 0;
    output[1] = relation ? relation->trust : 0;
    output[2] = relation ? relation->respect : 0;
    output[3] = relation ? relation->familiarity : 0;
    return 1;
}
