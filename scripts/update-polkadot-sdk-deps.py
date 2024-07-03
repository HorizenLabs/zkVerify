"""
A script to update Polkadot-SDK dependencies used within zkVerify.

Usage example:
    python3 scripts/update-polkadot-sdk-deps.py release-crates-io-v1.5.0
"""

import argparse
import git
import tempfile
import os
import subprocess
import tomlkit # pip install tomlkit

def parse_arguments():
    """
    Parse command-line arguments and return the parsed arguments.
    """
    parser = argparse.ArgumentParser(description="Update zkVerify dependencies coming from Polkadot-SDK")
    parser.add_argument("branch", help="name of the Polkadot-SDK branch to update dependencies to")
    parser.add_argument("--no-check", action='store_true', help="skip 'cargo check' after updating dependencies")
    parser.add_argument("--no-commit", action='store_true', help="skip committing after updating dependencies")
    parser.add_argument("--verbose", action='store_true', help="verbose logging")
    return parser.parse_args()

def print_verbose(verbose, message):
    """
    Print verbose message if enabled
    """
    if (verbose):
        print(message)        


if __name__ == "__main__":
    # Parse command-line arguments
    args = parse_arguments()

    with tempfile.TemporaryDirectory() as tmp_dir:
        git_url = "git@github.com:paritytech/polkadot-sdk.git"
        print(f"Temporary cloning Polkadot-SDK branch {args.branch} into {tmp_dir}...")
        try:
            repo_polkadot = git.Repo.clone_from(git_url, tmp_dir, branch=args.branch, depth=1)
        except Exception as e:
            print(f"...error cloning! The script will exit now!")
            exit(-1)
        print("...cloning done!")
        commit_hash = repo_polkadot.head.commit
        print(f"Using commit with hash: {commit_hash}.")

        polkadot_libs = {}
        workspace_file_path_polkadot = f"{tmp_dir}/Cargo.toml"
        print_verbose(args.verbose, "Polkadot-SDK libraries:")
        with open(workspace_file_path_polkadot) as workspace_file_polkadot:
            workspace_toml_polkadot = tomlkit.load(workspace_file_polkadot)
            for library_path in workspace_toml_polkadot["workspace"]["members"]:
                library_file = f"{tmp_dir}/{library_path}/Cargo.toml"
                with open(library_file) as library_file:
                    library_toml = tomlkit.load(library_file)
                    polkadot_libs[library_toml['package']['name']] = library_toml['package']['version']
                    print_verbose(args.verbose, f"  Name: {library_toml['package']['name']} - Version: {library_toml['package']['version']}")

        print(f"Checking zkVerify libraries to update...")
        workspace_file_path_zkverify = f"{os.getcwd()}/Cargo.toml"
        zkverify_deps = {}
        with open(workspace_file_path_zkverify) as workspace_file_zkverify:
            workspace_toml_zkverify = tomlkit.load(workspace_file_zkverify)
            for library_name in workspace_toml_zkverify["workspace"]["dependencies"]:
                if (library_name in polkadot_libs):
                    library_info = workspace_toml_zkverify["workspace"]["dependencies"][library_name]
                    if (type(library_info) is tomlkit.items.InlineTable and
                        "version" in library_info):
                        # e.g.: my_lib = { version = "0.1.0", default-features = false }
                        version = library_info["version"]
                        library_info["version"] = polkadot_libs[library_name]
                    elif (type(library_info) is tomlkit.items.String):
                        # e.g.: my_lib = "0.1.0"
                        version = library_info
                        workspace_toml_zkverify["workspace"]["dependencies"][library_name] = polkadot_libs[library_name]
                    else:
                        print(f"WARNING: unable to determine library version for {library_name}!")
                    if version != polkadot_libs[library_name]:
                        print(f"{library_name} is going to be updated (from {version} to {polkadot_libs[library_name]})")
                    else:
                        print_verbose(args.verbose, f"{library_name} is not going to be updated")
        print(f"...checking done!")

        print(f"Updating Cargo.toml file...")
        workspace_file_path = f"{os.getcwd()}/Cargo.toml"
        with open(workspace_file_path_zkverify, 'w') as workspace_file_zkverify:
            workspace_file_zkverify.writelines(tomlkit.dumps(workspace_toml_zkverify))
        print(f"...updating done!")

        if (not args.no_check):
            print(f"Checking with cargo...")
            try:
                completed_process = subprocess.run("cargo check", shell=True, executable="/bin/bash")
            except Exception as e:
                print(f"...error checking! The script will exit now!")
                exit(-1)
            if completed_process.returncode != 0:
                print(f"...failure checking! The script will exit now!")
                exit(-1)
            print(f"...checking done!")

        if (not args.no_commit):
            repo_zkverify = git.Repo(os.getcwd())
            print(f"Committing to zkVerify repository...")
            commit_message = f"Bump dependencies to Polkadot-SDK branch {args.branch} (commit {commit_hash})"
            print(f"Using commit message: {commit_message}.")
            try:
                repo_zkverify.index.add("Cargo.toml")
                repo_zkverify.index.commit(commit_message)
            except Exception as e:
                print(f"...error committing! The script will exit now!")
                exit(-1)
            print(f"...committing done!")