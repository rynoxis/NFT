import { RedspotUserConfig } from 'redspot/types'
import '@redspot/patract'
import '@redspot/chai'
import '@redspot/gas-reporter'

export default {
    defaultNetwork: 'development',
    contract: {
        ink: {
            toolchain: 'nightly',
            sources: ['uniswap-v2/contracts/**']
        }
    },
    networks: {
        development: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/',
            types: {
                UniswapV2PairPairError: {
                    _enum: {
                        PSP22Error:' PSP22Error',
                        OwnableError: 'OwnableError',
                        PausableError: 'PausableError',
                        K: null,
                        InsufficientLiquidityMinted: null,
                        InsufficientLiquidityBurned: null,
                        InsufficientOutputAmount: null,
                        InsufficientLiquidity: null,
                        InsufficientInputAmount: null,
                        SafeTransferFailed: null,
                        InvalidTo: null,
                        Overflow: null,
                        Locked: null,
                        SubUnderFlow1: null,
                        SubUnderFlow2: null,
                        SubUnderFlow3: null,
                        SubUnderFlow4: null,
                        SubUnderFlow5: null,
                        SubUnderFlow6: null,
                        SubUnderFlow7: null,
                        SubUnderFlow8: null,
                        SubUnderFlow9: null,
                        SubUnderFlow10: null,
                        SubUnderFlow11: null,
                        SubUnderFlow12: null,
                        SubUnderFlow13: null,
                        SubUnderFlow14: null,
                        MulOverFlow1: null,
                        MulOverFlow2: null,
                        MulOverFlow3: null,
                        MulOverFlow4: null,
                        MulOverFlow5: null,
                        MulOverFlow6: null,
                        MulOverFlow7: null,
                        MulOverFlow8: null,
                        MulOverFlow9: null,
                        MulOverFlow10: null,
                        MulOverFlow11: null,
                        MulOverFlow12: null,
                        MulOverFlow13: null,
                        MulOverFlow14: null,
                        MulOverFlow15: null,
                        DivByZero1: null,
                        DivByZero2: null,
                        DivByZero3: null,
                        DivByZero4: null,
                        DivByZero5: null,
                        AddOverflow1: null,
                    },
                },
                PSP22Error: {
                    _enum: {
                        Custom: 'String',
                        InsufficientBalance: null,
                        InsufficientAllowance: null,
                        ZeroRecipientAddress: null,
                        ZeroSenderAddress: null,
                        SafeTransferCheckFailed: 'String',
                    }
                },
                OwnableError: {
                    _enum: {
                        CallerIsNotOwner: null,
                        NewOwnerIsZero: null,
                    }
                },
                PausableError: {
                    _enum: {
                        Paused: null,
                        NotPaused: null,
                    }
                },
                OpenbrushContractsErrorsPsp22Psp22Error: {
                    _enum: {
                        Custom: 'String',
                        InsufficientBalance: null,
                        InsufficientAllowance: null,
                        ZeroRecipientAddress: null,
                        ZeroSenderAddress: null,
                        SafeTransferCheckFailed: 'String',
                    }
                }
            },
        },
        substrate: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            accounts: ['//Alice'],
        }
    },
    mocha: {
        timeout: 600000
    }
} as RedspotUserConfig