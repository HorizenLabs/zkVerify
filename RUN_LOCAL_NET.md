# Run local net

This branch is useful to simulate a net with 10 validators and run some node upgrade.

As example of how to use it imagine that you have a chain version A that should be upgraded to version B.

1. Create a new branch `upgrade_node_test` from this one `10_validators_local_net`.
2. Rebase the branch/ref that contains the version that you would obtain after the update onto `upgrade_node_test`.
3. If you need more than 10 validators you should modify `chain_spech.rs`, the compose file and add the needed resources
4. Now tag the _old_ docker image (the one that you would upgrade) with a new clear name like `prev`.
    For instance if you would upgrade the `horizenlabs/zkverify:latest` docker image issue the follow command

    ```sh
    docker tag horizenlabs/zkverify:latest horizenlabs/zkverify:prev
    ```

5. Save the new `chain_spec.json` to configure a valid local network:

   ```sh
   docker run -ti --rm --entrypoint "nh-node" horizenlabs/zkverify:prev build-spec --chain local --raw > docker/resources/chain_local.json
   ```

6. Start the local network with the current image. The compose file assume that you used `horizenlabs/zkverify:prev` docker image

   ```sh
   docker compose -f docker/dockerfiles/zkv-docker-compose-update.yaml up -d
   ```

7. Compile the new chain and create the new `horizenlabs/zkverify:latest` docker image:

   ```sh
   . cfg
   bootstrap.sh
   ```

8. Upgrade just 3 validators and the RPC one: stop these containers, change the image reference in the compose file from `prev` to `latest` and starts them again

   ```sh
   docker compose -f docker/dockerfiles/zkv-docker-compose-update.yaml stop node_local node_01 node_02 node_03
   docker compose -f docker/dockerfiles/zkv-docker-compose-update.yaml up node_local node_01 node_02 node_03
   ```

9. Now you can upgrade the runtime.
10. When you want to upgrade all nodes just change the compose's image and stop and up them again.

When you're done use the follow command to stop all

```sh
docker compose -f docker/dockerfiles/zkv-docker-compose-update.yaml down
```

You would see a container logs use `docker compose logs <name>` (`--help` for the commands) or if you would check what image a container use `docker ps` command.