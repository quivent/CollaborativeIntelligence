# Probabilistic Graphical Model Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a Bayesian network where agents, capabilities, and knowledge are modeled as random variables with probabilistic dependencies, enabling uncertainty quantification and belief propagation.

## Probabilistic Model Foundation

### Core Concepts
```python
BAYESIAN_NETWORK_COMPONENTS = {
    'random_variables': 'Agents, capabilities, tasks, outcomes, contexts',
    'conditional_dependencies': 'P(X|Parents(X)) for each variable X',
    'prior_distributions': 'P(X) for variables without parents',
    'evidence': 'Observed values and performance data',
    'inference_engine': 'Belief propagation and sampling algorithms',
    'learning_algorithms': 'Parameter learning from agent behavior data'
}

VARIABLE_TYPES = {
    'discrete': 'Categorical variables (agent_type, task_category, success_level)',
    'continuous': 'Numerical variables (performance_score, complexity_rating)',
    'binary': 'Boolean variables (task_success, capability_present)',
    'ordinal': 'Ordered categories (experience_level, difficulty_level)'
}

DEPENDENCY_TYPES = {
    'causal': 'X directly influences Y',
    'evidential': 'X provides evidence about Y',
    'confounding': 'X and Y share common causes',
    'mediating': 'X influences Y through intermediate variables'
}
```

## Data Extraction Process

### Step 1: Random Variable Identification
```python
def identify_random_variables():
    """
    Identify all random variables from agent data
    """
    variables = {
        'agent_variables': {},
        'capability_variables': {},
        'task_variables': {},
        'performance_variables': {},
        'context_variables': {},
        'outcome_variables': {}
    }
    
    agent_dirs = [d for d in Path('../AGENTS').iterdir() 
                  if d.is_dir() and d.name[0].isupper()]
    
    for agent_dir in agent_dirs:
        agent_name = agent_dir.name
        
        # Extract agent-specific variables
        agent_vars = extract_agent_variables(agent_dir, agent_name)
        variables['agent_variables'].update(agent_vars)
        
        # Extract capability variables
        capability_vars = extract_capability_variables(agent_dir, agent_name)
        variables['capability_variables'].update(capability_vars)
        
        # Extract task and performance variables
        task_perf_vars = extract_task_performance_variables(agent_dir, agent_name)
        variables['task_variables'].update(task_perf_vars['tasks'])
        variables['performance_variables'].update(task_perf_vars['performance'])
        
        # Extract context variables
        context_vars = extract_context_variables(agent_dir, agent_name)
        variables['context_variables'].update(context_vars)
    
    # Extract outcome variables from collaboration data
    outcome_vars = extract_outcome_variables()
    variables['outcome_variables'].update(outcome_vars)
    
    return variables

def extract_agent_variables(agent_dir, agent_name):
    """
    Extract agent-specific random variables
    """
    agent_variables = {}
    
    # Agent expertise level variable
    expertise_level = infer_agent_expertise_level(agent_dir, agent_name)
    agent_variables[f'{agent_name}_expertise_level'] = {
        'type': 'ordinal',
        'domain': ['novice', 'intermediate', 'advanced', 'expert'],
        'observed_value': expertise_level,
        'confidence': calculate_expertise_confidence(agent_dir, agent_name),
        'metadata': {
            'agent': agent_name,
            'variable_type': 'agent_attribute',
            'description': f'Expertise level of agent {agent_name}'
        }
    }
    
    # Agent activity level variable
    activity_level = calculate_agent_activity_level(agent_dir, agent_name)
    agent_variables[f'{agent_name}_activity_level'] = {
        'type': 'continuous',
        'domain': [0.0, 1.0],
        'observed_value': activity_level,
        'distribution': 'beta',
        'parameters': estimate_activity_distribution_params(agent_dir),
        'metadata': {
            'agent': agent_name,
            'variable_type': 'agent_behavior',
            'description': f'Activity level of agent {agent_name}'
        }
    }
    
    # Agent domain specialization variables
    domain_specializations = extract_domain_specializations(agent_dir, agent_name)
    for domain, specialization_level in domain_specializations.items():
        var_name = f'{agent_name}_specialization_{domain}'
        agent_variables[var_name] = {
            'type': 'continuous',
            'domain': [0.0, 1.0],
            'observed_value': specialization_level,
            'distribution': 'beta',
            'metadata': {
                'agent': agent_name,
                'domain': domain,
                'variable_type': 'domain_specialization',
                'description': f'{agent_name} specialization in {domain}'
            }
        }
    
    # Agent collaboration propensity
    collaboration_propensity = calculate_collaboration_propensity(agent_name)
    agent_variables[f'{agent_name}_collaboration_propensity'] = {
        'type': 'continuous',
        'domain': [0.0, 1.0],
        'observed_value': collaboration_propensity,
        'distribution': 'beta',
        'metadata': {
            'agent': agent_name,
            'variable_type': 'social_behavior',
            'description': f'Propensity of {agent_name} to collaborate'
        }
    }
    
    return agent_variables

def extract_capability_variables(agent_dir, agent_name):
    """
    Extract capability-related random variables
    """
    capability_variables = {}
    
    # Extract capabilities from agent documentation
    capabilities = extract_agent_capabilities_with_levels(agent_dir, agent_name)
    
    for capability_name, capability_data in capabilities.items():
        # Capability presence variable (does agent have this capability?)
        var_name = f'{agent_name}_has_{capability_name}'
        capability_variables[var_name] = {
            'type': 'binary',
            'domain': [True, False],
            'observed_value': True,  # If extracted, agent has it
            'confidence': capability_data.get('confidence', 0.8),
            'metadata': {
                'agent': agent_name,
                'capability': capability_name,
                'variable_type': 'capability_presence',
                'description': f'Whether {agent_name} has {capability_name} capability'
            }
        }
        
        # Capability proficiency variable (how good is agent at this capability?)
        proficiency_var = f'{agent_name}_proficiency_{capability_name}'
        capability_variables[proficiency_var] = {
            'type': 'ordinal',
            'domain': ['basic', 'competent', 'proficient', 'expert'],
            'observed_value': capability_data.get('level', 'competent'),
            'metadata': {
                'agent': agent_name,
                'capability': capability_name,
                'variable_type': 'capability_proficiency',
                'description': f'Proficiency of {agent_name} in {capability_name}'
            }
        }
    
    return capability_variables

def extract_task_performance_variables(agent_dir, agent_name):
    """
    Extract task and performance-related variables
    """
    task_variables = {}
    performance_variables = {}
    
    # Analyze working files for task execution data
    working_files = list(agent_dir.glob('working_*.md'))
    
    for working_file in working_files:
        session_id = extract_session_id_from_filename(working_file.name)
        
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract task information
        tasks = extract_tasks_from_session(content, session_id)
        
        for task_id, task_data in tasks.items():
            # Task complexity variable
            complexity_var = f'task_{task_id}_complexity'
            task_variables[complexity_var] = {
                'type': 'ordinal',
                'domain': ['simple', 'moderate', 'complex', 'very_complex'],
                'observed_value': task_data.get('complexity', 'moderate'),
                'metadata': {
                    'task_id': task_id,
                    'session_id': session_id,
                    'variable_type': 'task_attribute',
                    'description': f'Complexity of task {task_id}'
                }
            }
            
            # Task domain variable
            domain_var = f'task_{task_id}_domain'
            task_variables[domain_var] = {
                'type': 'discrete',
                'domain': ['technical', 'creative', 'analytical', 'business', 'general'],
                'observed_value': task_data.get('domain', 'general'),
                'metadata': {
                    'task_id': task_id,
                    'session_id': session_id,
                    'variable_type': 'task_attribute',
                    'description': f'Domain of task {task_id}'
                }
            }
            
            # Performance variables
            if task_data.get('completed', False):
                # Task success variable
                success_var = f'{agent_name}_success_{task_id}'
                performance_variables[success_var] = {
                    'type': 'binary',
                    'domain': [True, False],
                    'observed_value': task_data.get('success', True),
                    'metadata': {
                        'agent': agent_name,
                        'task_id': task_id,
                        'variable_type': 'performance_outcome',
                        'description': f'Success of {agent_name} on task {task_id}'
                    }
                }
                
                # Performance quality variable
                quality_var = f'{agent_name}_quality_{task_id}'
                performance_variables[quality_var] = {
                    'type': 'continuous',
                    'domain': [0.0, 1.0],
                    'observed_value': task_data.get('quality_score', 0.7),
                    'distribution': 'beta',
                    'metadata': {
                        'agent': agent_name,
                        'task_id': task_id,
                        'variable_type': 'performance_quality',
                        'description': f'Quality of {agent_name} performance on task {task_id}'
                    }
                }
    
    return {'tasks': task_variables, 'performance': performance_variables}

def extract_context_variables(agent_dir, agent_name):
    """
    Extract contextual variables that might influence performance
    """
    context_variables = {}
    
    # Time of day context (if temporal patterns exist)
    temporal_patterns = analyze_temporal_activity_patterns(agent_dir, agent_name)
    if temporal_patterns:
        time_preference_var = f'{agent_name}_time_preference'
        context_variables[time_preference_var] = {
            'type': 'discrete',
            'domain': ['morning', 'afternoon', 'evening', 'night'],
            'observed_value': temporal_patterns.get('preferred_time', 'afternoon'),
            'metadata': {
                'agent': agent_name,
                'variable_type': 'temporal_context',
                'description': f'Preferred working time for {agent_name}'
            }
        }
    
    # Collaboration context
    collaboration_history = analyze_collaboration_context(agent_name)
    if collaboration_history:
        collab_context_var = f'{agent_name}_collaboration_context'
        context_variables[collab_context_var] = {
            'type': 'discrete',
            'domain': ['solo', 'pair', 'small_group', 'large_group'],
            'observed_value': collaboration_history.get('preferred_mode', 'pair'),
            'metadata': {
                'agent': agent_name,
                'variable_type': 'collaboration_context',
                'description': f'Preferred collaboration mode for {agent_name}'
            }
        }
    
    return context_variables
```

### Step 2: Dependency Structure Learning
```python
def learn_dependency_structure(variables):
    """
    Learn the dependency structure of the Bayesian network
    """
    # Flatten all variables
    all_variables = {}
    for var_type, var_dict in variables.items():
        all_variables.update(var_dict)
    
    # Learn dependencies using domain knowledge and data analysis
    dependencies = {
        'expert_knowledge': define_expert_knowledge_dependencies(),
        'data_driven': learn_data_driven_dependencies(all_variables),
        'causal_assumptions': define_causal_assumptions(),
        'temporal_dependencies': learn_temporal_dependencies(all_variables)
    }
    
    # Combine dependency sources
    final_structure = combine_dependency_sources(dependencies, all_variables)
    
    return final_structure

def define_expert_knowledge_dependencies():
    """
    Define dependencies based on domain expertise
    """
    expert_dependencies = [
        # Agent expertise influences task success
        {
            'parent': 'agent_expertise_level',
            'child': 'task_success',
            'type': 'causal',
            'strength': 'strong',
            'rationale': 'Higher expertise leads to higher success probability'
        },
        
        # Task complexity influences success probability
        {
            'parent': 'task_complexity',
            'child': 'task_success',
            'type': 'causal',
            'strength': 'strong',
            'rationale': 'More complex tasks are harder to complete successfully'
        },
        
        # Agent-task domain match influences performance
        {
            'parent': ['agent_domain_specialization', 'task_domain'],
            'child': 'performance_quality',
            'type': 'causal',
            'strength': 'moderate',
            'rationale': 'Domain match improves performance quality'
        },
        
        # Collaboration context influences individual performance
        {
            'parent': 'collaboration_context',
            'child': 'individual_performance',
            'type': 'contextual',
            'strength': 'moderate',
            'rationale': 'Collaboration mode affects individual contributions'
        },
        
        # Agent activity level influences task engagement
        {
            'parent': 'agent_activity_level',
            'child': 'task_engagement',
            'type': 'causal',
            'strength': 'moderate',
            'rationale': 'More active agents engage more in tasks'
        }
    ]
    
    return expert_dependencies

def learn_data_driven_dependencies(all_variables):
    """
    Learn dependencies from actual agent data
    """
    data_dependencies = []
    
    # Prepare data matrix
    data_matrix, variable_names = prepare_data_matrix(all_variables)
    
    if data_matrix.shape[0] > 10:  # Need sufficient data
        # Use statistical tests for dependency detection
        dependencies = detect_statistical_dependencies(data_matrix, variable_names)
        
        # Convert to standardized format
        for dep in dependencies:
            data_dependencies.append({
                'parent': dep['parent_var'],
                'child': dep['child_var'],
                'type': 'statistical',
                'strength': dep['correlation_strength'],
                'p_value': dep['p_value'],
                'test_statistic': dep['test_statistic']
            })
    
    return data_dependencies

def construct_bayesian_network(variables, dependencies):
    """
    Construct the Bayesian network structure
    """
    # Create network structure
    network = {
        'variables': {},
        'parents': {},
        'children': {},
        'cpds': {},  # Conditional Probability Distributions
        'structure_score': 0.0
    }
    
    # Add all variables
    for var_name, var_data in variables.items():
        network['variables'][var_name] = var_data
        network['parents'][var_name] = []
        network['children'][var_name] = []
    
    # Add dependencies
    for dependency in dependencies:
        parent_vars = dependency['parent'] if isinstance(dependency['parent'], list) else [dependency['parent']]
        child_var = dependency['child']
        
        # Find actual variable names that match patterns
        actual_parents = find_matching_variables(parent_vars, variables)
        actual_children = find_matching_variables([child_var], variables)
        
        for parent in actual_parents:
            for child in actual_children:
                if parent in network['variables'] and child in network['variables']:
                    network['parents'][child].append(parent)
                    network['children'][parent].append(child)
    
    # Learn CPDs for each variable
    for var_name in network['variables']:
        cpd = learn_conditional_probability_distribution(
            var_name, 
            network['parents'][var_name],
            variables
        )
        network['cpds'][var_name] = cpd
    
    # Calculate structure score
    network['structure_score'] = calculate_network_score(network, variables)
    
    return network

def learn_conditional_probability_distribution(var_name, parent_vars, all_variables):
    """
    Learn CPD for a variable given its parents
    """
    var_data = all_variables[var_name]
    
    if not parent_vars:
        # No parents - learn prior distribution
        return learn_prior_distribution(var_data)
    else:
        # Has parents - learn conditional distribution
        return learn_conditional_distribution(var_name, parent_vars, all_variables)

def learn_prior_distribution(var_data):
    """
    Learn prior distribution for a variable without parents
    """
    var_type = var_data['type']
    
    if var_type == 'binary':
        # Beta-Bernoulli conjugate prior
        observed_value = var_data.get('observed_value', True)
        confidence = var_data.get('confidence', 0.8)
        
        # Convert confidence to beta parameters
        if observed_value:
            alpha = confidence * 10
            beta = (1 - confidence) * 10
        else:
            alpha = (1 - confidence) * 10
            beta = confidence * 10
        
        return {
            'distribution_type': 'beta_bernoulli',
            'parameters': {'alpha': alpha, 'beta': beta},
            'mean': alpha / (alpha + beta)
        }
    
    elif var_type == 'discrete':
        domain = var_data['domain']
        observed_value = var_data.get('observed_value')
        
        # Dirichlet-Multinomial conjugate prior
        # Give higher weight to observed value
        alpha = np.ones(len(domain))
        if observed_value and observed_value in domain:
            idx = domain.index(observed_value)
            alpha[idx] = 10
        
        return {
            'distribution_type': 'dirichlet_multinomial',
            'parameters': {'alpha': alpha.tolist()},
            'domain': domain,
            'probabilities': (alpha / np.sum(alpha)).tolist()
        }
    
    elif var_type == 'continuous':
        observed_value = var_data.get('observed_value', 0.5)
        domain = var_data.get('domain', [0.0, 1.0])
        
        # Normal or Beta distribution
        if domain == [0.0, 1.0]:
            # Beta distribution for [0,1] variables
            mean = observed_value
            variance = 0.1  # Default variance
            
            # Convert to beta parameters
            alpha = mean * ((mean * (1 - mean)) / variance - 1)
            beta = (1 - mean) * ((mean * (1 - mean)) / variance - 1)
            
            return {
                'distribution_type': 'beta',
                'parameters': {'alpha': max(1, alpha), 'beta': max(1, beta)},
                'mean': mean
            }
        else:
            # Normal distribution
            return {
                'distribution_type': 'normal',
                'parameters': {'mu': observed_value, 'sigma': 1.0},
                'mean': observed_value
            }
    
    elif var_type == 'ordinal':
        domain = var_data['domain']
        observed_value = var_data.get('observed_value')
        
        # Ordered categorical distribution
        probabilities = np.ones(len(domain)) * 0.1
        if observed_value and observed_value in domain:
            idx = domain.index(observed_value)
            probabilities[idx] = 0.7
        
        # Normalize
        probabilities = probabilities / np.sum(probabilities)
        
        return {
            'distribution_type': 'ordered_categorical',
            'parameters': {'probabilities': probabilities.tolist()},
            'domain': domain,
            'probabilities': probabilities.tolist()
        }

def learn_conditional_distribution(var_name, parent_vars, all_variables):
    """
    Learn conditional distribution P(var_name | parent_vars)
    """
    var_data = all_variables[var_name]
    var_type = var_data['type']
    
    # Create conditional probability table structure
    parent_domains = []
    for parent_var in parent_vars:
        parent_data = all_variables[parent_var]
        if parent_data['type'] == 'continuous':
            # Discretize continuous variables for CPT
            parent_domains.append(['low', 'medium', 'high'])
        else:
            parent_domains.append(parent_data['domain'])
    
    # Generate all parent configurations
    parent_configs = list(itertools.product(*parent_domains)) if parent_domains else [()]
    
    # Learn distribution for each parent configuration
    conditional_distributions = {}
    
    for config in parent_configs:
        config_key = '_'.join(str(c) for c in config) if config else 'no_parents'
        
        # For now, use prior distribution modified by parent influence
        base_distribution = learn_prior_distribution(var_data)
        
        # Modify distribution based on parent configuration
        modified_distribution = modify_distribution_by_parents(
            base_distribution, config, parent_vars, var_name, all_variables
        )
        
        conditional_distributions[config_key] = modified_distribution
    
    return {
        'distribution_type': 'conditional',
        'parent_variables': parent_vars,
        'parent_domains': parent_domains,
        'conditional_distributions': conditional_distributions
    }
```

### Step 3: Inference Engine Implementation
```python
def implement_inference_engine(network):
    """
    Implement inference algorithms for the Bayesian network
    """
    inference_engine = {
        'exact_inference': create_exact_inference(network),
        'approximate_inference': create_approximate_inference(network),
        'prediction': create_prediction_functions(network),
        'explanation': create_explanation_functions(network)
    }
    
    return inference_engine

def create_exact_inference(network):
    """
    Create exact inference algorithms (variable elimination, junction tree)
    """
    def variable_elimination(query_vars, evidence=None):
        """
        Perform exact inference using variable elimination
        """
        if evidence is None:
            evidence = {}
        
        # Create factor graph
        factors = create_initial_factors(network, evidence)
        
        # Eliminate variables not in query
        elimination_order = determine_elimination_order(
            network, query_vars, evidence
        )
        
        for var_to_eliminate in elimination_order:
            factors = eliminate_variable(var_to_eliminate, factors)
        
        # Normalize result
        result_factor = normalize_factor(factors[0])
        
        return {
            'query_variables': query_vars,
            'posterior_distribution': result_factor,
            'evidence': evidence,
            'method': 'variable_elimination'
        }
    
    def belief_propagation(query_vars, evidence=None, max_iterations=100):
        """
        Perform belief propagation on the network
        """
        if evidence is None:
            evidence = {}
        
        # Initialize messages
        messages = initialize_messages(network)
        
        # Iterative message passing
        for iteration in range(max_iterations):
            old_messages = copy.deepcopy(messages)
            
            # Update all messages
            for var in network['variables']:
                messages = update_variable_messages(var, messages, network, evidence)
            
            # Check convergence
            if messages_converged(old_messages, messages):
                break
        
        # Compute marginals
        marginals = {}
        for query_var in query_vars:
            marginals[query_var] = compute_marginal(query_var, messages, network)
        
        return {
            'query_variables': query_vars,
            'marginal_distributions': marginals,
            'evidence': evidence,
            'iterations': iteration + 1,
            'method': 'belief_propagation'
        }
    
    return {
        'variable_elimination': variable_elimination,
        'belief_propagation': belief_propagation
    }

def create_approximate_inference(network):
    """
    Create approximate inference algorithms (MCMC, variational)
    """
    def gibbs_sampling(query_vars, evidence=None, num_samples=1000, burn_in=100):
        """
        Perform Gibbs sampling for approximate inference
        """
        if evidence is None:
            evidence = {}
        
        # Initialize sample
        current_sample = initialize_sample(network, evidence)
        samples = []
        
        for iteration in range(num_samples + burn_in):
            # Sample each non-evidence variable
            for var in network['variables']:
                if var not in evidence:
                    current_sample[var] = sample_variable_given_markov_blanket(
                        var, current_sample, network
                    )
            
            # Store sample after burn-in
            if iteration >= burn_in:
                samples.append(current_sample.copy())
        
        # Compute empirical distributions
        empirical_distributions = {}
        for query_var in query_vars:
            empirical_distributions[query_var] = compute_empirical_distribution(
                query_var, samples, network
            )
        
        return {
            'query_variables': query_vars,
            'empirical_distributions': empirical_distributions,
            'evidence': evidence,
            'num_samples': num_samples,
            'method': 'gibbs_sampling'
        }
    
    def importance_sampling(query_vars, evidence=None, num_samples=1000):
        """
        Perform importance sampling
        """
        if evidence is None:
            evidence = {}
        
        samples = []
        weights = []
        
        for _ in range(num_samples):
            # Sample from proposal distribution
            sample = sample_from_proposal(network, evidence)
            
            # Compute importance weight
            weight = compute_importance_weight(sample, network, evidence)
            
            samples.append(sample)
            weights.append(weight)
        
        # Normalize weights
        weights = np.array(weights)
        weights = weights / np.sum(weights)
        
        # Compute weighted empirical distributions
        weighted_distributions = {}
        for query_var in query_vars:
            weighted_distributions[query_var] = compute_weighted_empirical_distribution(
                query_var, samples, weights, network
            )
        
        return {
            'query_variables': query_vars,
            'weighted_distributions': weighted_distributions,
            'evidence': evidence,
            'effective_sample_size': 1.0 / np.sum(weights**2),
            'method': 'importance_sampling'
        }
    
    return {
        'gibbs_sampling': gibbs_sampling,
        'importance_sampling': importance_sampling
    }

def create_prediction_functions(network):
    """
    Create prediction functions for various scenarios
    """
    def predict_task_success(agent_name, task_characteristics, context=None):
        """
        Predict probability of task success for an agent
        """
        if context is None:
            context = {}
        
        # Identify relevant variables
        query_vars = [f'{agent_name}_success_task']
        evidence = {
            f'{agent_name}_expertise_level': get_agent_expertise(agent_name, network),
            'task_complexity': task_characteristics.get('complexity', 'moderate'),
            'task_domain': task_characteristics.get('domain', 'general')
        }
        evidence.update(context)
        
        # Perform inference
        inference_result = network['inference_engine']['exact_inference']['variable_elimination'](
            query_vars, evidence
        )
        
        return {
            'agent': agent_name,
            'task_characteristics': task_characteristics,
            'success_probability': inference_result['posterior_distribution'],
            'evidence_used': evidence
        }
    
    def predict_collaboration_success(agent_list, task_characteristics, context=None):
        """
        Predict success probability for multi-agent collaboration
        """
        if context is None:
            context = {}
        
        # Model collaboration as joint probability
        individual_predictions = []
        
        for agent in agent_list:
            agent_prediction = predict_task_success(agent, task_characteristics, context)
            individual_predictions.append(agent_prediction)
        
        # Combine individual predictions (simplified model)
        combined_probability = combine_collaboration_probabilities(
            individual_predictions, agent_list, network
        )
        
        return {
            'collaborating_agents': agent_list,
            'task_characteristics': task_characteristics,
            'collaboration_success_probability': combined_probability,
            'individual_predictions': individual_predictions
        }
    
    def predict_optimal_agent_assignment(task_characteristics, available_agents, context=None):
        """
        Predict optimal agent assignment for a task
        """
        if context is None:
            context = {}
        
        # Predict success for each agent
        agent_predictions = []
        
        for agent in available_agents:
            prediction = predict_task_success(agent, task_characteristics, context)
            agent_predictions.append({
                'agent': agent,
                'success_probability': prediction['success_probability'],
                'prediction_details': prediction
            })
        
        # Rank agents by predicted success
        agent_predictions.sort(
            key=lambda x: x['success_probability'].get('mean', 0.5), 
            reverse=True
        )
        
        return {
            'task_characteristics': task_characteristics,
            'agent_rankings': agent_predictions,
            'recommended_agent': agent_predictions[0]['agent'] if agent_predictions else None
        }
    
    return {
        'predict_task_success': predict_task_success,
        'predict_collaboration_success': predict_collaboration_success,
        'predict_optimal_agent_assignment': predict_optimal_agent_assignment
    }

def create_explanation_functions(network):
    """
    Create explanation functions for model interpretability
    """
    def explain_prediction(query_result, sensitivity_analysis=True):
        """
        Explain why a particular prediction was made
        """
        explanation = {
            'prediction': query_result,
            'key_factors': [],
            'sensitivity_analysis': {},
            'counterfactuals': []
        }
        
        # Identify key factors
        evidence = query_result.get('evidence', {})
        for var, value in evidence.items():
            factor_importance = calculate_factor_importance(var, value, query_result, network)
            explanation['key_factors'].append({
                'variable': var,
                'value': value,
                'importance_score': factor_importance
            })
        
        # Sort by importance
        explanation['key_factors'].sort(
            key=lambda x: x['importance_score'], 
            reverse=True
        )
        
        # Sensitivity analysis
        if sensitivity_analysis:
            for factor in explanation['key_factors'][:3]:  # Top 3 factors
                sensitivity = perform_sensitivity_analysis(
                    factor['variable'], query_result, network
                )
                explanation['sensitivity_analysis'][factor['variable']] = sensitivity
        
        # Generate counterfactuals
        counterfactuals = generate_counterfactuals(query_result, network)
        explanation['counterfactuals'] = counterfactuals
        
        return explanation
    
    def analyze_model_assumptions(network):
        """
        Analyze and report on model assumptions
        """
        analysis = {
            'independence_assumptions': [],
            'conditional_dependencies': [],
            'model_limitations': [],
            'confidence_assessment': {}
        }
        
        # Check independence assumptions
        for var in network['variables']:
            parents = network['parents'][var]
            non_parents = [v for v in network['variables'] if v not in parents and v != var]
            
            for non_parent in non_parents:
                independence_assumption = f"{var} ⊥ {non_parent} | {', '.join(parents)}"
                analysis['independence_assumptions'].append(independence_assumption)
        
        # Analyze conditional dependencies
        for var in network['variables']:
            parents = network['parents'][var]
            if parents:
                dependency = {
                    'child': var,
                    'parents': parents,
                    'strength': estimate_dependency_strength(var, parents, network)
                }
                analysis['conditional_dependencies'].append(dependency)
        
        return analysis
    
    return {
        'explain_prediction': explain_prediction,
        'analyze_model_assumptions': analyze_model_assumptions
    }
```

### Step 4: Model Validation and Learning
```python
def validate_and_learn_model(network, validation_data=None):
    """
    Validate model performance and learn from new data
    """
    validation_results = {
        'structure_validation': validate_network_structure(network),
        'parameter_validation': validate_model_parameters(network),
        'predictive_validation': validate_predictive_performance(network, validation_data),
        'learning_results': {}
    }
    
    # Online learning from new observations
    if validation_data:
        learning_results = perform_online_learning(network, validation_data)
        validation_results['learning_results'] = learning_results
    
    return validation_results

def validate_network_structure(network):
    """
    Validate the structure of the Bayesian network
    """
    structure_validation = {
        'is_dag': check_directed_acyclic_graph(network),
        'connectivity': analyze_network_connectivity(network),
        'complexity_metrics': calculate_complexity_metrics(network),
        'domain_consistency': check_domain_consistency(network)
    }
    
    return structure_validation

def perform_online_learning(network, new_observations):
    """
    Update network parameters based on new observations
    """
    learning_results = {
        'observations_processed': len(new_observations),
        'parameter_updates': {},
        'structure_changes': [],
        'performance_improvement': {}
    }
    
    for observation in new_observations:
        # Update CPDs based on new observation
        for var, value in observation.items():
            if var in network['cpds']:
                updated_cpd = update_cpd_with_observation(
                    network['cpds'][var], 
                    var, 
                    value, 
                    observation
                )
                
                learning_results['parameter_updates'][var] = {
                    'old_parameters': network['cpds'][var]['parameters'],
                    'new_parameters': updated_cpd['parameters'],
                    'update_magnitude': calculate_parameter_change(
                        network['cpds'][var], updated_cpd
                    )
                }
                
                network['cpds'][var] = updated_cpd
    
    return learning_results

def export_probabilistic_model(network, inference_engine, validation_results):
    """
    Export the complete probabilistic model
    """
    # Export network structure
    network_export = {
        'variables': serialize_variables(network['variables']),
        'dependencies': serialize_dependencies(network),
        'cpds': serialize_cpds(network['cpds']),
        'structure_score': network['structure_score'],
        'metadata': {
            'creation_timestamp': datetime.now().isoformat(),
            'model_type': 'bayesian_network',
            'num_variables': len(network['variables']),
            'num_dependencies': sum(len(parents) for parents in network['parents'].values())
        }
    }
    
    with open('bayesian_network.json', 'w') as f:
        json.dump(network_export, f, indent=2, default=str)
    
    # Export inference capabilities
    inference_export = {
        'available_algorithms': list(inference_engine.keys()),
        'algorithm_details': {
            'exact_inference': ['variable_elimination', 'belief_propagation'],
            'approximate_inference': ['gibbs_sampling', 'importance_sampling'],
            'prediction': ['task_success', 'collaboration_success', 'optimal_assignment'],
            'explanation': ['prediction_explanation', 'model_assumptions']
        }
    }
    
    with open('inference_capabilities.json', 'w') as f:
        json.dump(inference_export, f, indent=2)
    
    # Export validation results
    with open('model_validation.json', 'w') as f:
        json.dump(validation_results, f, indent=2, default=str)
    
    # Export model for external use (e.g., pgmpy format)
    pgmpy_export = convert_to_pgmpy_format(network)
    with open('network_pgmpy.pkl', 'wb') as f:
        pickle.dump(pgmpy_export, f)
    
    # Create model summary report
    summary_report = create_model_summary_report(network, validation_results)
    with open('model_summary_report.md', 'w') as f:
        f.write(summary_report)

def create_model_summary_report(network, validation_results):
    """
    Create human-readable model summary report
    """
    report = f"""# Probabilistic Graphical Model Summary

## Model Overview
- **Model Type**: Bayesian Network
- **Number of Variables**: {len(network['variables'])}
- **Number of Dependencies**: {sum(len(parents) for parents in network['parents'].values())}
- **Structure Score**: {network['structure_score']:.4f}

## Variable Types
"""
    
    # Count variable types
    var_type_counts = {}
    for var_data in network['variables'].values():
        var_type = var_data['type']
        var_type_counts[var_type] = var_type_counts.get(var_type, 0) + 1
    
    for var_type, count in var_type_counts.items():
        report += f"- **{var_type}**: {count} variables\n"
    
    report += f"""
## Key Dependencies
"""
    
    # List important dependencies
    important_deps = []
    for child, parents in network['parents'].items():
        if parents:
            dep_strength = estimate_dependency_strength(child, parents, network)
            important_deps.append((child, parents, dep_strength))
    
    # Sort by strength and show top 10
    important_deps.sort(key=lambda x: x[2], reverse=True)
    
    for child, parents, strength in important_deps[:10]:
        report += f"- **{child}** depends on {', '.join(parents)} (strength: {strength:.3f})\n"
    
    report += f"""
## Model Validation
- **Structure Valid**: {validation_results['structure_validation']['is_dag']}
- **Network Connectivity**: {validation_results['structure_validation']['connectivity']['connected']}
- **Parameter Consistency**: All CPDs properly normalized

## Inference Capabilities
- Exact inference via variable elimination and belief propagation
- Approximate inference via Gibbs sampling and importance sampling
- Predictive modeling for task success and agent assignment
- Model explanation and assumption analysis
"""
    
    return report
```

## Implementation Steps

1. **Variable Identification**: Extract random variables from agent data with proper domains
2. **Dependency Learning**: Learn causal and statistical dependencies between variables
3. **Network Construction**: Build Bayesian network structure with CPDs
4. **Inference Implementation**: Implement exact and approximate inference algorithms
5. **Prediction Functions**: Create agent performance and task assignment predictions
6. **Model Validation**: Validate structure, parameters, and predictive performance
7. **Export System**: Save model in multiple formats for analysis and deployment

## Output Files
- `bayesian_network.json`: Complete network structure and parameters
- `inference_capabilities.json`: Available inference algorithms and functions
- `model_validation.json`: Validation results and performance metrics
- `network_pgmpy.pkl`: Network in pgmpy format for external analysis
- `model_summary_report.md`: Human-readable model documentation

## Inference Query Examples
```python
# Predict task success probability
success_prob = inference_engine['prediction']['predict_task_success'](
    'Developer', 
    {'complexity': 'high', 'domain': 'technical'},
    {'collaboration_context': 'pair'}
)

# Find optimal agent for task
optimal_agent = inference_engine['prediction']['predict_optimal_agent_assignment'](
    {'complexity': 'moderate', 'domain': 'creative'},
    ['Designer', 'Artist', 'Developer']
)

# Explain prediction
explanation = inference_engine['explanation']['explain_prediction'](success_prob)
```