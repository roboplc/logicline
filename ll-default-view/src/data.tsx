/*
 * Sample LD
 *
 * 1 -7---- 2 -- 4 -- 11 --8--- 5 ---- 0
 *       |     |       |   |   |       |
 *       |     `-9-----'   |   |       |
 *       |-- 3 ------------'   `-- 6 --'
 *       |                 |
 *       `---12 -----------'
 */

import { LLData } from "logicline-view";

export const data: LLData = {
  lines: {
    line1: {
      name: "line1",
      steps: [
        {
          name: "№1/false",
          input: {
            a: {
              a: 85,
              d: 67,
            },
            b: [{ a: 85 }, { d: 67 }, { e: 85 }],
            c: 45,
            d: 67,
            e: 85,
            f: 123,
            g: 45,
            h: 67,
            i: 4,
            j: 5,
            k: 11,
            l: 10,
            m: 12,
            n: 15,
          },
          passed: true,
          input_kind: "external",
        },
        [
          {
            name: "№2/false",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: false,
            input_kind: "external",
          },
          {
            name: "№3/false",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: true,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 85,
            b: 123,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line2: {
      name: "line2",
      steps: [
        {
          name: "№1/true",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: true,
          input_kind: "external",
        },
        [
          {
            name: "№2/true",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: true,
            input_kind: "external",
          },
          {
            name: "№3/false",
            input: {
              a: 850000000000000000000,
              b: 123,
              c: 45,
              d: 67,
            },
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 85,
            b: 123,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line3: {
      name: "line3",
      steps: [
        {
          name: "№1/true",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: true,
          input_kind: "external",
        },
        [
          {
            name: "№2/false",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: false,
            input_kind: "external",
          },
          {
            name: "№3/true",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: true,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false12",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 85,
            b: 123,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line4: {
      name: "line4",
      steps: [
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage/true",
          input: {
            a: {
              a: 85,
              d: 67,
            },
            b: [{ a: 85 }, { d: 67 }, { e: 85 }],
            c: 45,
            d: 67,
            e: 85,
            f: 123,
            g: 45,
            h: 67,
            i: 4,
            j: 5,
            k: 11,
            l: 10,
            m: 12,
            n: 15,
          },
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
        },
        {
          name: "voltage/true",
          input: null,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
        },
        {
          name: "voltage/true",
          input: null,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line5: {
      name: "line5xxx1",
      steps: [
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: true,
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: false,
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
      ],
    },
  },
};

export const data2: AviData = {
  lines: {
    line1: {
      name: "line1",
      steps: [
        {
          name: "№1/false",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№2/false",
            input: {
              a: {
                a: 85,
                d: 67,
              },
            },
            passed: false,
            input_kind: "external",
          },
          {
            name: "№3/false",
            input: {
              a: 850000000000000000000,
            },
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: {
              a: 850000000000000000000,
              b: 123,
              c: 45,
              d: 67,
            },
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line2: {
      name: "line2",
      steps: [
        {
          name: "№1/true",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: true,
          input_kind: "external",
        },
        [
          {
            name: "№2/true",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: true,
            input_kind: "external",
          },
          {
            name: "№3/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 85,
            b: 123,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line3: {
      name: "line3",
      steps: [
        {
          name: "№1/true",
          input: {
            a: 850000000000000000000,
            b: 123,
            c: 45,
            d: 67,
          },
          passed: true,
          input_kind: "external",
        },
        [
          {
            name: "№2/false",
            input: {
              a: {
                a: 85,
                d: 67,
              },
              b: [{ a: 85 }, { d: 67 }, { e: 85 }],
              c: 45,
              d: 67,
              e: 85,
              f: 123,
              g: 45,
              h: 67,
              i: 4,
              j: 5,
              k: 11,
              l: 10,
              m: 12,
              n: 15,
            },
            passed: false,
            input_kind: "external",
          },
          {
            name: "№3/false",
            input: {
              a: 850000000000000000000,
              b: 123,
              c: 45,
              d: 67,
            },
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false12",
          input: null,
          passed: false,
        },
        {
          name: "№5/false",
          input: {
            a: 85,
            b: 123,
          },
          passed: false,
          input_kind: "external",
        },
        [
          {
            name: "№6/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "№7/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line4: {
      name: "line4",
      steps: [
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
        },
        {
          name: "voltage/true",
          input: null,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
        },
        {
          name: "voltage/true",
          input: null,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
        {
          name: "temperature_critical/true",
          input: 95,
          passed: true,
        },
        {
          name: "voltage/true",
          input: null,
          passed: true,
          input_kind: "external",
        },
        {
          name: "voltage_critical/false",
          input: 205,
          passed: false,
        },
        {
          name: "OFF/false",
          input: null,
          passed: false,
        },
      ],
    },
    line5: {
      name: "line5",
      steps: [
        [
          {
            name: "voltage/false1",
            input: null,
            passed: true,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: true,
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: true,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: true,
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
        [
          {
            name: "voltage/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
          {
            name: "voltage_critical/false",
            input: null,
            passed: false,
            input_kind: "external",
          },
        ],
      ],
    },
  },
};
