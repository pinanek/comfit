"use client";

import { type ComponentProps, useId } from "react";
import styles from "./logo.module.css";

export function Logo({ className, ...props }: ComponentProps<"svg">) {
  const clipId = useId();

  return (
    <svg
      viewBox="0 0 765 244"
      width="765"
      height="244"
      className={[styles.root, className].filter(Boolean).join(" ")}
      data-logo=""
      aria-hidden="true"
      {...props}
    >
      <defs>
        <clipPath id={clipId}>
          <path d="M 66 67 L 174 67 C 176 81 188 85 196 98 C 203 108 204 125 204 148 L 204 179 C 204 203 192 216 168 216 L 72 216 C 48 216 36 202 36 179 L 36 145 C 36 122 37 108 44 98 C 52 85 64 81 66 67 Z" />
        </clipPath>
      </defs>
      <g>
        <path
          d="M 66 67 L 174 67 C 176 81 188 85 196 98 C 203 108 204 125 204 148 L 204 179 C 204 203 192 216 168 216 L 72 216 C 48 216 36 202 36 179 L 36 145 C 36 122 37 108 44 98 C 52 85 64 81 66 67 Z"
          fill="var(--c-cpn-surface0)"
          fillOpacity="0.72"
        />
        <g clipPath={`url(#${clipId})`}>
          <path
            fill="var(--c-cpn-pink)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(85 113) rotate(-25) scale(0.94)"
          />
          <path
            fill="var(--c-cpn-yellow)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(113 103) rotate(15) scale(0.92)"
          />
          <path
            fill="var(--c-cpn-green)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(141 107) rotate(-18) scale(0.98)"
          />
          <path
            fill="var(--c-cpn-peach)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(164 119) rotate(52) scale(0.94)"
          />
          <path
            fill="var(--c-cpn-mauve)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(72 136) rotate(60) scale(0.95)"
          />
          <path
            fill="var(--c-cpn-red)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(99 130) rotate(-24) scale(0.95)"
          />
          <path
            fill="var(--c-cpn-teal)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(128 130) rotate(12) scale(1.0)"
          />
          <path
            fill="var(--c-cpn-lavender)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(157 137) rotate(-45) scale(1.05)"
          />
          <path
            fill="var(--c-cpn-yellow)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(177 146) rotate(80) scale(0.92)"
          />
          <path
            fill="var(--c-cpn-green)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(67 164) rotate(-45) scale(1.04)"
          />
          <path
            fill="var(--c-cpn-sky)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(91 159) rotate(15) scale(1.02)"
          />
          <path
            fill="var(--c-cpn-rosewater)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(119 153) rotate(-35) scale(1.05)"
          />
          <path
            fill="var(--c-cpn-pink)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(147 163) rotate(36) scale(1.0)"
          />
          <path
            fill="var(--c-cpn-blue)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(174 167) rotate(-18) scale(1.0)"
          />
          <path
            fill="var(--c-cpn-yellow)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(70 189) rotate(32) scale(0.97)"
          />
          <path
            fill="var(--c-cpn-peach)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(99 184) rotate(-35) scale(1.0)"
          />
          <path
            fill="var(--c-cpn-mauve)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(126 184) rotate(12) scale(1.07)"
          />
          <path
            fill="var(--c-cpn-red)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(153 190) rotate(-30) scale(1.05)"
          />
          <path
            fill="var(--c-cpn-teal)"
            d="M -13.5 -8 C -20.5 -5 -19.5 4.8 -12.5 8.5 C -4 12.5 8 11.5 13 6.5 C 20 1.5 17.5 -7.5 9 -9.5 C 0 -11.8 -8 -11.5 -13.5 -8 Z"
            transform="translate(176 188) rotate(46) scale(0.92)"
          />
        </g>
        <path
          d="M 66 67 L 174 67 C 176 81 188 85 196 98 C 203 108 204 125 204 148 L 204 179 C 204 203 192 216 168 216 L 72 216 C 48 216 36 202 36 179 L 36 145 C 36 122 37 108 44 98 C 52 85 64 81 66 67 Z"
          fill="none"
          stroke="var(--c-cpn-text)"
          strokeWidth="6.5"
          strokeLinejoin="round"
        />
        <path
          d="M 50 120 Q 48 143 49 174"
          fill="none"
          stroke="var(--c-cpn-base)"
          strokeWidth="5"
          strokeLinecap="round"
          opacity=".86"
        />
        <path
          d="M 190 120 Q 192 149 190 168"
          fill="none"
          stroke="var(--c-cpn-base)"
          strokeWidth="3"
          strokeLinecap="round"
          opacity=".58"
        />
        <rect
          x="60"
          y="61"
          width="120"
          height="14"
          rx="6"
          fill="var(--c-cpn-surface2)"
          stroke="var(--c-cpn-text)"
          strokeWidth="4"
        />
        <path
          d="M 60 28 H 180 Q 192 28 192 40 V 53 Q 192 65 180 65 H 60 Q 48 65 48 53 V 40 Q 48 28 60 28 Z"
          fill="var(--c-cpn-mauve)"
          stroke="var(--c-cpn-text)"
          strokeWidth="6"
          strokeLinejoin="round"
        />
        <path d="M 65 40 H 175" stroke="var(--c-cpn-base)" strokeOpacity=".66" strokeWidth="4" strokeLinecap="round" />
      </g>
      <g>
        <path
          d="M62 273Q62 353 96.5 416.5Q131 480 193.0 515.5Q255 551 333 551Q451 551 529 461Q539 450 539 435Q539 413 517 398Q507 391 494 391Q471 391 455 409Q430 436 400.5 448.5Q371 461 333 461Q256 461 208.0 409.5Q160 358 160 273Q160 219 182.5 176.5Q205 134 245.5 110.0Q286 86 338 86Q406 86 448 121Q467 135 485 135Q499 135 511 126Q530 110 530 90Q530 75 518 63Q446 -4 338 -4Q259 -4 196.0 32.0Q133 68 97.5 131.5Q62 195 62 273Z"
          fill="var(--c-cpn-text)"
          transform="translate(263.00 162) scale(0.145 -0.145)"
        />
        <path
          d="M62 273Q62 354 97.0 417.0Q132 480 195.0 515.5Q258 551 338 551Q417 551 479.5 515.5Q542 480 577.5 417.0Q613 354 613 273Q613 193 578.0 130.0Q543 67 480.5 31.5Q418 -4 338 -4Q258 -4 195.0 31.5Q132 67 97.0 130.0Q62 193 62 273ZM515 273Q515 327 492.5 370.0Q470 413 429.5 437.0Q389 461 338 461Q287 461 246.5 437.0Q206 413 183.0 370.0Q160 327 160 273Q160 219 183.0 176.5Q206 134 246.5 110.0Q287 86 338 86Q389 86 429.5 110.0Q470 134 492.5 176.5Q515 219 515 273Z"
          fill="var(--c-cpn-pink)"
          transform="translate(349.13 162) scale(0.145 -0.145)"
        />
        <path
          d="M859 336V50Q859 28 845.0 13.5Q831 -1 808 -1Q786 -1 771.5 13.5Q757 28 757 50V336Q757 397 726.5 428.5Q696 460 645 460Q593 460 559.5 424.0Q526 388 526 332V50Q526 28 512.0 13.5Q498 -1 475 -1Q453 -1 438.5 13.5Q424 28 424 50V336Q424 397 393.5 428.5Q363 460 312 460Q259 460 225.5 424.0Q192 388 192 332V50Q192 28 178.0 13.5Q164 -1 141 -1Q119 -1 104.5 13.5Q90 28 90 50V495Q90 518 104.5 532.0Q119 546 141 546Q163 546 177.0 532.5Q191 519 192 497Q218 523 254.0 537.5Q290 552 332 552Q383 552 423.5 531.5Q464 511 490 473Q520 510 565.0 531.0Q610 552 665 552Q752 552 805.5 495.0Q859 438 859 336Z"
          fill="var(--c-cpn-text)"
          transform="translate(445.99 162) scale(0.145 -0.145)"
        />
        <path
          d="M202 554V527H304Q325 527 338.0 515.0Q351 503 351 483Q351 465 338.0 453.0Q325 441 304 441H202V51Q202 28 188.0 14.0Q174 0 149 0Q129 0 114.5 14.0Q100 28 100 51V441H57Q36 441 23.0 453.0Q10 465 10 483Q10 503 23.0 515.0Q36 527 57 527H100V544Q100 615 128.5 668.5Q157 722 208.5 751.5Q260 781 326 781Q348 781 362.5 768.0Q377 755 377 735Q377 715 362.5 702.0Q348 689 326 689Q270 689 236.0 652.0Q202 615 202 554Z"
          fill="var(--c-cpn-text)"
          transform="translate(580.98 162) scale(0.145 -0.145)"
        />
        <path
          d="M100 51V496Q100 519 114.5 533.0Q129 547 151 547Q174 547 188.0 533.0Q202 519 202 496V51Q202 29 188.0 14.5Q174 0 151 0Q129 0 114.5 14.5Q100 29 100 51ZM85 702Q85 729 104.5 748.5Q124 768 151 768Q178 768 197.5 748.5Q217 729 217 702Q217 675 197.5 655.5Q178 636 151 636Q124 636 104.5 655.5Q85 675 85 702Z"
          fill="var(--c-cpn-text)"
          transform="translate(633.18 162) scale(0.145 -0.145)"
        />
        <path
          d="M366 50Q366 28 349.5 14.0Q333 0 307 0H289Q240 0 199.5 25.5Q159 51 136.0 95.0Q113 139 113 194V441H67Q46 441 33.0 453.0Q20 465 20 483Q20 503 33.0 515.0Q46 527 67 527H113V671Q113 693 127.0 707.0Q141 721 163 721Q185 721 199.0 707.0Q213 693 213 671V527H298Q319 527 332.0 515.0Q345 503 345 483Q345 465 332.0 453.0Q319 441 298 441H213V194Q213 154 235.0 127.0Q257 100 289 100H323Q341 100 353.5 86.0Q366 72 366 50Z"
          fill="var(--c-cpn-text)"
          transform="translate(675.96 162) scale(0.145 -0.145)"
        />
      </g>
    </svg>
  );
}
