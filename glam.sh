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
# KAMINO
#
KAMINO_IDL=$(realpath ../glam/anchor/deps/kamino_lending/kamino_lending.json)
KAMINO_OUT=../glam/anchor/programs/glam/src/cpi_autogen/kamino_lending.rs

$CPI_GEN $KAMINO_IDL \
    --ixs initUserMetadata \
    --ixs initObligation \
    --ixs initObligationFarmsForReserve \
    --ixs depositReserveLiquidityAndObligationCollateral \
    > $KAMINO_OUT
