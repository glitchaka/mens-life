#ifndef MSPA_C_API_H
#define MSPA_C_API_H
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

typedef struct mspa_runtime mspa_runtime;
typedef enum mspa_status { MSPA_OK=0, MSPA_INVALID_ARGUMENT, MSPA_UNKNOWN_AGENT, MSPA_DUPLICATE_AGENT, MSPA_INVALID_STATE, MSPA_INVALID_PACK, MSPA_QUOTA_EXCEEDED, MSPA_VERSION_MISMATCH, MSPA_NAMESPACE_CONFLICT, MSPA_TIME_REGRESSION, MSPA_BATCH_CONFLICT, MSPA_OUT_OF_MEMORY, MSPA_UNSUPPORTED_OPERATION, MSPA_SERIALIZATION_ERROR, MSPA_INTERNAL_ERROR } mspa_status;
typedef struct mspa_agent_id { uint64_t world; uint64_t local_id; } mspa_agent_id;
typedef struct mspa_runtime_config { uint64_t runtime_id,world_id,time_domain_id,ticks_per_second_num,ticks_per_second_den,seed_root; int diagnostics_enabled; } mspa_runtime_config;
typedef struct mspa_proposition_arg { const char* role; const char* value; int has_agent; mspa_agent_id agent; int has_concept; uint64_t concept_id; } mspa_proposition_arg;
typedef struct mspa_observe_input { mspa_agent_id agent; uint64_t predicate_id,namespace_id; int positive; const mspa_proposition_arg* arguments; size_t argument_count; uint64_t process_time; int has_event_time; uint64_t event_time; float perceived_certainty,source_trust; } mspa_observe_input;
typedef struct mspa_observe_result { int state_changed,decision_recommended,current_action_threatened; uint32_t urgency; uint64_t diagnostic_token; } mspa_observe_result;
typedef struct mspa_need_signal { uint64_t concept_id; float pressure; } mspa_need_signal;
typedef struct mspa_named_signal { const char* name; float value; } mspa_named_signal;
typedef struct mspa_expected_effect { uint32_t channel; uint64_t concept_id; mspa_agent_id target; float magnitude; const char* outcome_tag; } mspa_expected_effect;
typedef struct mspa_action_option { uint64_t type_id,instance_id,continuity_key; float cost,risk,progress,novelty,social_exposure,interruption_cost,context_stability; const char* const* context_tags; size_t context_tag_count; const mspa_expected_effect* expected_effects; size_t expected_effect_count; const mspa_named_signal* signals; size_t signal_count; } mspa_action_option;
typedef struct mspa_decision_result { int has_selection; uint64_t selected_action; mspa_agent_id selected_target; float confidence; } mspa_decision_result;
typedef struct mspa_buffer { uint8_t* data; size_t size; } mspa_buffer;

uint32_t mspa_version_major(void); uint32_t mspa_version_minor(void); uint32_t mspa_version_patch(void);
mspa_status mspa_runtime_create(const mspa_runtime_config*,mspa_runtime** out_runtime); void mspa_runtime_destroy(mspa_runtime*);
mspa_status mspa_agent_create(mspa_runtime*,mspa_agent_id,uint64_t process_time); mspa_status mspa_agent_remove(mspa_runtime*,mspa_agent_id,uint64_t process_time); int mspa_agent_has(const mspa_runtime*,mspa_agent_id);
mspa_status mspa_observe(mspa_runtime*,const mspa_observe_input*,mspa_observe_result*); mspa_status mspa_decide(mspa_runtime*,mspa_agent_id,const mspa_action_option*,size_t,uint64_t process_time,uint64_t current_continuity,mspa_decision_result*); mspa_status mspa_decide_ex(mspa_runtime*,mspa_agent_id,const mspa_action_option*,size_t,const mspa_need_signal*,size_t,const mspa_named_signal*,size_t,float urgency,uint64_t process_time,uint64_t current_continuity,mspa_decision_result*);
mspa_status mspa_report(mspa_runtime*,mspa_agent_id,uint64_t action_type,int success,uint64_t process_time); mspa_status mspa_report_ex(mspa_runtime*,mspa_agent_id,uint64_t action_type,int success,const char* const* context_tags,size_t context_count,const char* const* outcome_tags,size_t outcome_count,uint64_t process_time); mspa_status mspa_advance(mspa_runtime*,mspa_agent_id,uint64_t target_time);
mspa_status mspa_agent_export(mspa_runtime*,mspa_agent_id,uint64_t exported_at,mspa_buffer*); mspa_status mspa_agent_import(mspa_runtime*,const uint8_t*,size_t,mspa_agent_id,uint64_t import_time); void mspa_buffer_free(mspa_buffer*);
const char* mspa_last_error(void);
#ifdef __cplusplus
}
#endif
#endif
