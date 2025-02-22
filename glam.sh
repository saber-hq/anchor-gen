#!/bin/bash

CPI_GEN="cargo run -p glam-cpi-gen glam --config ../glam/anchor/programs/glam/src/cpi_autogen/config.yaml"

#
# DRIFT
#
DRIFT_IDL=$(realpath ../glam/anchor/deps/drift/drift.json)
DRIFT_OUT=../glam/anchor/programs/glam/src/cpi_autogen/drift.rs

# placeOrders excluded because we do extra authz checks
# --ixs placeOrders \

$CPI_GEN $DRIFT_IDL \
    --ixs initializeUserStats \
    --ixs initializeUser \
    --ixs deleteUser \
    --ixs updateUserCustomMarginRatio \
    --ixs updateUserDelegate \
    --ixs updateUserMarginTradingEnabled \
    --ixs deposit \
    --ixs withdraw \
    --ixs cancelOrders \
    --ixs cancelOrdersByIds \
    --ixs modifyOrder \
    > $DRIFT_OUT

#
# JUPITER GOV
#
JUP_GOV_IDL=$(realpath ../glam/anchor/deps/jupiter_gov/govern.json)
JUP_GOV_OUT=../glam/anchor/programs/glam/src/cpi_autogen/jupiter_gov.rs
JUP_VOTE_IDL=$(realpath ../glam/anchor/deps/jupiter_vote/locked_voter.json)
JUP_VOTE_OUT=../glam/anchor/programs/glam/src/cpi_autogen/jupiter_vote.rs

# toggleMaxLock excluded because we do custom authz

$CPI_GEN $JUP_VOTE_IDL --idl-name-alias jupiter_vote \
    --ixs newEscrow \
    --ixs increaseLockedAmount \
    --ixs openPartialUnstaking \
    --ixs mergePartialUnstaking \
    --ixs withdraw \
    --ixs withdrawPartialUnstaking \
    --ixs castVote \
    > $JUP_VOTE_OUT

$CPI_GEN $JUP_GOV_IDL --idl-name-alias jupiter_gov \
    --ixs newVote \
    > $JUP_GOV_OUT

#
# KAMINO
#
KAMINO_IDL=$(realpath ../glam/anchor/deps/kamino_lending/kamino_lending.json)
KAMINO_OUT=../glam/anchor/programs/glam/src/cpi_autogen/kamino_lending.rs

# $CPI_GEN $KAMINO_IDL \
#     --ixs initUserMetadata \
#     --ixs initObligation \
#     --ixs initObligationFarmsForReserve \
#     --ixs depositReserveLiquidityAndObligationCollateral \
#     > $KAMINO_OUT
