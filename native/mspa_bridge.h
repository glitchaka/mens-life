#pragma once
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#define ML_NOEXCEPT noexcept
#else
#define ML_NOEXCEPT
#endif

typedef struct ml_brain ml_brain;

/* Task IDs follow the host's needs: bed, eat, shower, TV, toilet; -1 = rest. */
typedef struct ml_option {
    int32_t task;
    float cost;
    float progress;
} ml_option;

typedef struct ml_psych_state {
    float emotions[6]; /* joy, sadness, fear, anger, anxiety, aversion */
    float valence;
    float activation;
    float confidence;
    uint32_t memories;
    uint32_t beliefs;
    uint32_t learned_patterns;
    /* sociability, stability, empathy, persistence, curiosity, boldness */
    float personality[6];
    /* belief rigidity, goal persistence, switch resistance, threat sensitivity,
       social sensitivity, exploration */
    float derived_traits[6];
} ml_psych_state;

ml_brain* ml_brain_create(uint64_t seed, char* error, size_t capacity) ML_NOEXCEPT;
ml_brain* ml_brain_create_agent(uint64_t seed, uint64_t agent_id, char* error, size_t capacity) ML_NOEXCEPT;
void ml_brain_destroy(ml_brain* brain) ML_NOEXCEPT;
int32_t ml_brain_update(ml_brain* brain, uint64_t ticks, const float values[5]) ML_NOEXCEPT;
int32_t ml_brain_decide(ml_brain* brain, const ml_option* options, size_t count,
                        int32_t current, int32_t* selected) ML_NOEXCEPT;
/* kind: 0 started, 1 completed, 2 failed, 3 interrupted. */
int32_t ml_brain_event(ml_brain* brain, int32_t task, int32_t kind) ML_NOEXCEPT;
int32_t ml_brain_state(const ml_brain* brain, ml_psych_state* output) ML_NOEXCEPT;
void ml_brain_error(const ml_brain* brain, char* output, size_t capacity) ML_NOEXCEPT;
int32_t ml_brain_social_decide(ml_brain* brain, uint64_t target, int32_t* selected) ML_NOEXCEPT;
int32_t ml_brain_social_report(ml_brain* brain, uint64_t target, int32_t kind, int32_t success) ML_NOEXCEPT;
int32_t ml_brain_relation(const ml_brain* brain, uint64_t target, float output[4]) ML_NOEXCEPT;

#ifdef __cplusplus
}
#endif
#undef ML_NOEXCEPT
