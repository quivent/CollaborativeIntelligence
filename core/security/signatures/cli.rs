// Emergency CLI for signature system testing and management
// Critical security implementation for AUTH_BYPASS_001

use agent_signatures::SignatureManager;
use std::path::Path;

const REGISTRY_PATH: &str = "agent_signatures.json";

pub fn run_signature_cli() {
    println!("🔴 EMERGENCY AUTHENTICATION SYSTEM");
    println!("📋 Incident: AUTH_BYPASS_001 - Agent authentication bypass");
    println!("⚡ Priority: CRITICAL - System integrity compromised\n");

    let mut manager = SignatureManager::new();
    
    // Try to load existing registry
    if Path::new(REGISTRY_PATH).exists() {
        match manager.load_registry(REGISTRY_PATH) {
            Ok(_) => println!("✅ Loaded existing signature registry"),
            Err(e) => {
                println!("⚠️ Failed to load registry: {}", e);
                println!("🔧 Initializing new registry...");
                initialize_signatures(&mut manager);
            }
        }
    } else {
        println!("🔧 No existing registry found. Initializing emergency signatures...");
        initialize_signatures(&mut manager);
    }

    // Display system status
    println!("\n📊 SYSTEM STATUS:");
    println!("{}", manager.get_system_status());
    
    // Interactive commands
    loop {
        println!("\n🛡️ AUTHENTICATION COMMANDS:");
        println!("1. Verify agent identity");
        println!("2. Challenge agent");
        println!("3. System status");
        println!("4. List all agents");
        println!("5. Exit");
        
        print!("Enter command (1-5): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => verify_agent_interactive(&manager),
            "2" => challenge_agent_interactive(&manager),
            "3" => {
                println!("\n{}", manager.get_system_status());
            },
            "4" => list_agents(&manager),
            "5" => {
                println!("🔒 Authentication system CLI terminated");
                break;
            },
            _ => println!("❌ Invalid command. Please enter 1-5."),
        }
    }
}

fn initialize_signatures(manager: &mut SignatureManager) {
    match manager.initialize_emergency_signatures() {
        Ok(_) => {
            println!("✅ Emergency signatures generated for all agents");
            if let Err(e) = manager.save_registry(REGISTRY_PATH) {
                println!("⚠️ Failed to save registry: {}", e);
            } else {
                println!("💾 Registry saved to {}", REGISTRY_PATH);
            }
        },
        Err(e) => {
            println!("❌ Failed to initialize signatures: {:?}", e);
        }
    }
}

fn verify_agent_interactive(manager: &SignatureManager) {
    print!("Enter agent name to verify: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    let mut agent_name = String::new();
    std::io::stdin().read_line(&mut agent_name).unwrap();
    let agent_name = agent_name.trim();
    
    match manager.verify_agent(agent_name) {
        Ok(true) => {
            println!("✅ AUTHENTICATED: {} has valid signature", agent_name);
            if let Some(sig) = manager.get_agent_signature(agent_name) {
                println!("📝 Signature details:");
                println!("   Hash: {}", &sig.signature_hash[..16]);
                println!("   Timestamp: {}", sig.timestamp);
                println!("   Version: {}", sig.version);
            }
        },
        Ok(false) => println!("❌ AUTHENTICATION FAILED: {} has invalid signature", agent_name),
        Err(e) => println!("❌ ERROR: {:?}", e),
    }
}

fn challenge_agent_interactive(manager: &SignatureManager) {
    print!("Enter agent name to challenge: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    let mut agent_name = String::new();
    std::io::stdin().read_line(&mut agent_name).unwrap();
    let agent_name = agent_name.trim();
    
    println!("{}", manager.challenge_agent_identity(agent_name));
}

fn list_agents(manager: &SignatureManager) {
    println!("\n👥 REGISTERED AGENTS:");
    let mut agents: Vec<_> = manager.registry.signatures.keys().collect();
    agents.sort();
    
    for agent in agents {
        let status = match manager.verify_agent(agent) {
            Ok(true) => "✅ VALID",
            Ok(false) => "❌ INVALID", 
            Err(_) => "⚠️ ERROR",
        };
        println!("   {} - {}", agent, status);
    }
    
    println!("\nTotal agents: {}", manager.registry.signatures.len());
}

fn main() {
    run_signature_cli();
}