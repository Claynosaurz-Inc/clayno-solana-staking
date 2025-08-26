/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/clayno_staking.json`.
 */
export type ClaynoStaking = {
  "address": "CLAYrFKFaKo2JFed4HRVECG3Rb1iP3W44fcZmZmpWMH2",
  "metadata": {
    "name": "claynoStaking",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addEphemeralMultiplier",
      "discriminator": [
        156,
        80,
        82,
        172,
        26,
        176,
        144,
        168
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "multiplier",
          "type": "u8"
        },
        {
          "name": "expiryTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "addExperience",
      "discriminator": [
        173,
        71,
        194,
        172,
        34,
        175,
        28,
        106
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addMultiplier",
      "discriminator": [
        193,
        91,
        229,
        211,
        57,
        136,
        141,
        169
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "additionalMultiplier",
          "type": "u16"
        }
      ]
    },
    {
      "name": "claim",
      "discriminator": [
        62,
        198,
        214,
        193,
        213,
        159,
        108,
        210
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "createClass",
      "discriminator": [
        34,
        138,
        228,
        149,
        66,
        39,
        106,
        183
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "classPda",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  115,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint",
          "docs": [
            "NFT Accounts"
          ]
        },
        {
          "name": "tokenAccount"
        },
        {
          "name": "tokenMintRecord"
        },
        {
          "name": "systemProgram",
          "docs": [
            "Programs"
          ],
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "multiplier",
          "type": "u16"
        }
      ]
    },
    {
      "name": "increaseLevel",
      "discriminator": [
        110,
        109,
        239,
        156,
        104,
        121,
        101,
        3
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "modifyClass",
      "discriminator": [
        61,
        168,
        129,
        113,
        88,
        235,
        250,
        211
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "classPda",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  115,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint",
          "docs": [
            "NFT Accounts"
          ]
        },
        {
          "name": "tokenAccount"
        },
        {
          "name": "tokenMintRecord"
        },
        {
          "name": "systemProgram",
          "docs": [
            "Programs"
          ],
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "multiplier",
          "type": "u16"
        }
      ]
    },
    {
      "name": "reclaimRent",
      "discriminator": [
        218,
        200,
        19,
        197,
        227,
        89,
        192,
        22
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "removeEphemeralMultiplier",
      "discriminator": [
        123,
        172,
        56,
        53,
        164,
        84,
        172,
        81
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "removeExperience",
      "discriminator": [
        43,
        214,
        42,
        184,
        253,
        172,
        110,
        117
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "removeMultiplier",
      "discriminator": [
        55,
        214,
        71,
        213,
        128,
        1,
        183,
        120
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "address": "BAPw3n14PhrH5uyDymAN57qfKSogEYCayuF7r3NnTD33"
        },
        {
          "name": "user"
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "multiplier",
          "type": "u16"
        }
      ]
    },
    {
      "name": "stake",
      "discriminator": [
        206,
        176,
        202,
        18,
        200,
        209,
        179,
        108
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "auth",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  117,
                  116,
                  104
                ]
              }
            ]
          }
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "classPda",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  115,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "nft"
              }
            ]
          }
        },
        {
          "name": "nft",
          "docs": [
            "NFT Accounts"
          ]
        },
        {
          "name": "nftAccount",
          "writable": true
        },
        {
          "name": "nftEdition"
        },
        {
          "name": "nftRecord",
          "writable": true
        },
        {
          "name": "nftMetadata",
          "writable": true
        },
        {
          "name": "authRules"
        },
        {
          "name": "sysvarInstructions",
          "address": "Sysvar1nstructions1111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "docs": [
            "Programs"
          ],
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenMetadataProgram"
        },
        {
          "name": "authRulesProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "discriminator": [
        90,
        95,
        107,
        42,
        205,
        124,
        50,
        225
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "auth",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  117,
                  116,
                  104
                ]
              }
            ]
          }
        },
        {
          "name": "stakingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "classPda",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  115,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "nft"
              }
            ]
          }
        },
        {
          "name": "nft",
          "docs": [
            "NFT Accounts"
          ]
        },
        {
          "name": "nftAccount",
          "writable": true
        },
        {
          "name": "nftEdition"
        },
        {
          "name": "nftRecord",
          "writable": true
        },
        {
          "name": "nftMetadata",
          "writable": true
        },
        {
          "name": "authRules"
        },
        {
          "name": "sysvarInstructions",
          "address": "Sysvar1nstructions1111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "docs": [
            "Programs"
          ],
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenMetadataProgram"
        },
        {
          "name": "authRulesProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "class",
      "discriminator": [
        95,
        167,
        107,
        136,
        142,
        38,
        133,
        232
      ]
    },
    {
      "name": "stakingData",
      "discriminator": [
        66,
        164,
        234,
        202,
        189,
        107,
        3,
        93
      ]
    }
  ],
  "events": [
    {
      "name": "stakingAccountCreated",
      "discriminator": [
        32,
        131,
        46,
        201,
        50,
        21,
        62,
        235
      ]
    },
    {
      "name": "stakingAccountLevelUpdated",
      "discriminator": [
        19,
        148,
        176,
        5,
        11,
        141,
        142,
        105
      ]
    },
    {
      "name": "stakingAccountUpdated",
      "discriminator": [
        42,
        181,
        195,
        173,
        99,
        132,
        106,
        109
      ]
    },
    {
      "name": "claynoUpdated",
      "discriminator": [
        228,
        51,
        228,
        201,
        107,
        217,
        53,
        36
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6001,
      "name": "underflow",
      "msg": "Arithmetic underflow"
    },
    {
      "code": 6002,
      "name": "wrongCollection",
      "msg": "You passed an NFT with the wrong collection"
    },
    {
      "code": 6003,
      "name": "unverifiedCollection",
      "msg": "You passed an unverified collection"
    },
    {
      "code": 6004,
      "name": "invalidMetadata",
      "msg": "You passed an Invalid Metadata Account"
    },
    {
      "code": 6005,
      "name": "classPdaAlreadyExists",
      "msg": "The Class PDA for this NFT already exists"
    },
    {
      "code": 6006,
      "name": "wrongOwner",
      "msg": "The owner of this account is the wrong one"
    },
    {
      "code": 6007,
      "name": "wrongMint",
      "msg": "The mint of this account is the wrong one"
    },
    {
      "code": 6008,
      "name": "wrongAmount",
      "msg": "There are no NFTs in the account"
    },
    {
      "code": 6009,
      "name": "invalidRemainingAccountSchema",
      "msg": "The remaining accounts schema is not the correct one"
    },
    {
      "code": 6010,
      "name": "invalidTokenRecord",
      "msg": "The token record supplied is not valid"
    },
    {
      "code": 6011,
      "name": "neverStaked",
      "msg": "The user has never staked"
    },
    {
      "code": 6012,
      "name": "notEnoughPoints",
      "msg": "The user does not have enough points"
    },
    {
      "code": 6013,
      "name": "alreadyAtMaximumLevel",
      "msg": "The user is already at the maximum level"
    },
    {
      "code": 6014,
      "name": "notStaked",
      "msg": "The user does not have anything staked"
    },
    {
      "code": 6015,
      "name": "wrongAuthority",
      "msg": "The authority is not correct"
    },
    {
      "code": 6016,
      "name": "invalidExpiryTime",
      "msg": "The expiry time is not greater than the current time"
    },
    {
      "code": 6017,
      "name": "invalidMultiplier",
      "msg": "The multiplier needs to be greater than 1 (1x is the base multiplier already)"
    }
  ],
  "types": [
    {
      "name": "class",
      "docs": [
        "Represent a Class of a Claynosaurz"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "multiplier",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "ephemeralMultiplier",
      "docs": [
        "Represents a temporary multiplier with an expiry time."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "multiplier",
            "type": "u8"
          },
          {
            "name": "expiryTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakingAccountCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakingAccountLevelUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "points",
            "type": "u64"
          },
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakingAccountUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "points",
            "type": "u64"
          },
          {
            "name": "currentMultiplier",
            "type": "u16"
          },
          {
            "name": "ephemeralMultiplier",
            "type": {
              "vec": {
                "defined": {
                  "name": "ephemeralMultiplier"
                }
              }
            }
          },
          {
            "name": "lastClaimed",
            "type": "i64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "claynoUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "claynoId",
            "type": "pubkey"
          },
          {
            "name": "multiplier",
            "type": "u16"
          },
          {
            "name": "isStaked",
            "type": "bool"
          },
          {
            "name": "lockTime",
            "type": "i64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakingData",
      "docs": [
        "Represents the staking data for a user."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "currentLevel",
            "type": "u8"
          },
          {
            "name": "points",
            "type": "u64"
          },
          {
            "name": "currentMultiplier",
            "type": "u16"
          },
          {
            "name": "ephemeralMultiplier",
            "type": {
              "vec": {
                "defined": {
                  "name": "ephemeralMultiplier"
                }
              }
            }
          },
          {
            "name": "lastClaimed",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
