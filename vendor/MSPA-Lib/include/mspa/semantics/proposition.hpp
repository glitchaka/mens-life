#pragma once
#include <algorithm>
#include <map>
#include <string>
#include <vector>
#include "mspa/base/ids.hpp"
#include "mspa/base/time.hpp"
namespace mspa {
struct PropositionArg {
    std::string role{};
    std::string value{};
    bool has_agent{false}; AgentId agent{};
    bool has_concept{false}; ConceptId concept_id{};
};
struct Proposition {
    PredicateId predicate{};
    std::vector<PropositionArg> arguments{};
    bool positive{true};
    NamespaceId name_space{};
    std::map<std::string,std::string> qualifiers{};
    bool has_time{false}; PsychTime time{};
    std::string provenance{};
};
inline const PropositionArg* find_argument(const Proposition& p,const std::string& role) noexcept {
    auto it=std::find_if(p.arguments.begin(),p.arguments.end(),[&](const PropositionArg& a){return a.role==role;}); return it==p.arguments.end()?nullptr:&*it;
}
inline bool same_arguments(const Proposition& a,const Proposition& b) noexcept {
    if(a.arguments.size()!=b.arguments.size())return false;
    for(const auto& x:a.arguments){const auto* y=find_argument(b,x.role); if(!y||x.value!=y->value||x.has_agent!=y->has_agent||x.has_concept!=y->has_concept) return false; if(x.has_agent&&x.agent!=y->agent)return false; if(x.has_concept&&x.concept_id!=y->concept_id)return false;}
    return true;
}
inline bool equivalent(const Proposition&a,const Proposition&b) noexcept { return a.predicate==b.predicate&&a.name_space==b.name_space&&a.positive==b.positive&&same_arguments(a,b); }
inline bool contradicts(const Proposition&a,const Proposition&b) noexcept { return a.predicate==b.predicate&&a.name_space==b.name_space&&a.positive!=b.positive&&same_arguments(a,b); }
}
