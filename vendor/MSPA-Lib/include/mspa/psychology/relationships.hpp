#pragma once
#include "mspa/base/result.hpp"
#include "mspa/model/agent_state.hpp"
namespace mspa {struct RelationshipDelta{float affection{0},trust{0},respect{0},familiarity{0};};class RelationshipSystem{public:RelationshipState&get_or_create(AgentPsychState&,AgentId)const;const RelationshipState*find(const AgentPsychState&,AgentId)const noexcept;Result<void>adjust(AgentPsychState&,AgentId,const RelationshipDelta&)const;};}
