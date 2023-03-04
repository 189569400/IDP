import Icon from "@ant-design/icons"

const ShowPwd = () => (
  <svg width="1em" height="1em" viewBox="0 0 12 11" version="1.1">
    <title>打开</title>
    <defs>
      <filter color-interpolation-filters="auto" id="filter-1">
        <feColorMatrix
          in="SourceGraphic"
          type="matrix"
          values="0 0 0 0 0.094118 0 0 0 0 0.564706 0 0 0 0 1.000000 0 0 0 1.000000 0"
        ></feColorMatrix>
      </filter>
    </defs>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="新建数据源(弹框)" transform="translate(-872.000000, -387.000000)">
        <g id="编组-6" transform="translate(872.500000, 387.000000)">
          <rect
            id="矩形"
            fill="#D8D8D8"
            opacity="0"
            x="0"
            y="0"
            width="11"
            height="11"
          ></rect>
          <g filter="url(#filter-1)" id="眼睛">
            <g transform="translate(0.500000, 2.000000)">
              <path
                d="M9.92986445,2.92850573 C9.37357624,2.23794106 7.37541458,0 5,0 C2.62458542,0 0.639211993,2.23794106 0.0701355545,2.92850573 C-0.0233785182,3.04530477 -0.0233785182,3.21133902 0.0701355545,3.32813806 C0.629620821,4.01870273 2.62778248,6.25664379 5,6.25664379 C7.37221752,6.25664379 9.37357624,4.01870273 9.92986445,3.32813806 C10.0233785,3.21133902 10.0233785,3.04530477 9.92986445,2.92850573 Z M5,5.614035 C3.08176481,5.614035 1.40970313,3.91000274 0.735123758,3.12672336 C1.40970313,2.34344399 3.0977501,0.639411728 5,0.639411728 C6.9022499,0.639411728 8.59029687,2.34344399 9.27127037,3.12672336 C8.59029687,3.91000274 6.90224991,5.614035 5,5.614035 Z"
                id="形状"
                fill="#000000"
                fillRule="nonzero"
              ></path>
              <path
                d="M4.98401471,1.59852933 C4.15590763,1.59852933 3.4845942,2.26984275 3.4845942,3.09794984 C3.4845942,3.92605692 4.15590763,4.59737034 4.98401471,4.59737034 C5.81212179,4.59737034 6.48343522,3.92605692 6.48343522,3.09794984 C6.48343522,2.70027879 6.32546101,2.31889532 6.04426512,2.03769943 C5.76306923,1.75650353 5.38168575,1.59852933 4.98401471,1.59852933 Z M4.98401471,3.95795862 C4.63617363,3.95795862 4.32258322,3.74842421 4.18947019,3.42706095 C4.05635717,3.1056977 4.12993587,2.73579259 4.37589666,2.4898318 C4.62185745,2.24387101 4.99176256,2.17029231 5.31312581,2.30340532 C5.63448907,2.43651834 5.84402348,2.75010875 5.84402348,3.09794984 C5.84572758,3.3271442 5.75587262,3.54753704 5.59440462,3.71020555 C5.43293662,3.87287406 5.2132154,3.96435273 4.98401471,3.96435273 L4.98401471,3.95795862 Z"
                id="形状"
                fill="#000000"
                fillRule="nonzero"
              ></path>
            </g>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const showPwdIcon = (props) => <Icon component={ShowPwd} {...props} />

const hidePwd = () => (
  <svg width="1em" height="1em" viewBox="0 0 12 12" version="1.1">
    <title>关闭</title>
    <g
      id="页面-1"
      stroke="none"
      strokeWidth="1"
      fill="none"
      fillRule="evenodd"
    >
      <g id="新建数据源(弹框)" transform="translate(-872.000000, -361.000000)">
        <g id="编组-5" transform="translate(872.500000, 361.500000)">
          <rect
            id="矩形"
            fill="#D8D8D8"
            opacity="0"
            x="0"
            y="0"
            width="11"
            height="11"
          ></rect>
          <g
            id="编组-4"
            transform="translate(0.786814, 4.500000)"
            stroke="#1890FF"
            strokeLinecap="round"
            strokeWidth="0.7"
          >
            <path
              d="M0.203954547,0 C3.33510955,2.64666265 6.33510955,2.64666265 9.20395455,0"
              id="路径-30"
            ></path>
            <line
              x1="4.7163826"
              y1="1.98499699"
              x2="4.7163826"
              y2="3.63941173"
              id="路径-31"
            ></line>
            <line
              x1="6.30277891"
              y1="2.09582176"
              x2="7.12998628"
              y2="3.52858695"
              id="路径-31备份"
            ></line>
            <line
              x1="8"
              y1="1.39860067"
              x2="9.43276519"
              y2="2.22580804"
              id="路径-31备份-3"
            ></line>
            <line
              x1="3.12998628"
              y1="2.09582176"
              x2="2.30277891"
              y2="3.52858695"
              id="路径-31备份-2"
            ></line>
            <line
              x1="1.43276519"
              y1="1.39860067"
              x2="0"
              y2="2.22580804"
              id="路径-31备份-4"
            ></line>
          </g>
        </g>
      </g>
    </g>
  </svg>
)

const hidePwdIcon = (props) => <Icon component={hidePwd} {...props} />

const DataSetIcon = {
  hidePwdIcon,
  showPwdIcon,
}

export default DataSetIcon
