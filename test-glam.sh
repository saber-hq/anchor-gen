#!/bin/bash


CPI_GEN="cargo run -p glam-cpi-gen glam --config glam-tests/config.yaml"

#
# DRIFT
#
DRIFT_IDL=$(realpath ../glam/anchor/deps/drift/drift.json)
DRIFT_OUT=/tmp/drift.rs

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


diff $DRIFT_OUT glam-tests/drift-expected.rs > /dev/null

if [ $? -ne 0 ]; then
    echo "Test failed!"
    echo "Diff between generated and expected:"
    # diff $DRIFT_OUT glam-tests/drift-expected.rs 
    exit 1
else
    echo "Test passed!"
fi