#!/bin/bash
# Script to create CI command wrappers

# Make backup of original scripts
mkdir -p backup_ci_scripts
cp -f ci-init backup_ci_scripts/ci-init.orig
cp -f ci-fix backup_ci_scripts/ci-fix.orig
cp -f ci-verify backup_ci_scripts/ci-verify.orig

# Create init wrapper
cp ci-wrapper-template.sh ci-init.new
sed -i.bak 's/COMMAND_NAME="command"/COMMAND_NAME="init"/g' ci-init.new
chmod +x ci-init.new
mv ci-init.new ci-init
rm -f ci-init.new.bak

# Create fix wrapper
cp ci-wrapper-template.sh ci-fix.new
sed -i.bak 's/COMMAND_NAME="command"/COMMAND_NAME="fix"/g' ci-fix.new
chmod +x ci-fix.new
mv ci-fix.new ci-fix
rm -f ci-fix.new.bak

# Create verify wrapper
cp ci-wrapper-template.sh ci-verify.new
sed -i.bak 's/COMMAND_NAME="command"/COMMAND_NAME="verify"/g' ci-verify.new
chmod +x ci-verify.new
mv ci-verify.new ci-verify
rm -f ci-verify.new.bak

echo "CI command wrappers created successfully!"