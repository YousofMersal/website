use dioxus::prelude::*;

const PROFILE_PIC: manganis::ImageAsset = manganis::mg!(image("assets/profile.jpg"));

#[component]
pub fn Cv_page() -> Element {
    rsx! {
        Vitae {}
    }
}

#[component]
pub fn Vitae() -> Element {
    rsx! {

    div { class: "flex flex-col justify-start items-center overflow-hidden gap-2.5 px-[417px] bg-white",
        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-7",
            div { class: "flex flex-col justify-start items-center flex-grow-0 flex-shrink-0 gap-[45px] pt-8",
                div { class: "flex flex-col justify-between items-center flex-grow-0 flex-shrink-0 h-[295px] w-[250px] relative overflow-hidden",
                    img {
                        src: "profile.jpg",
                        class: "flex-grow-0 flex-shrink-0 w-[178px] h-[178px] rounded-[163px] object-cover"
                    }
                    p { class: "flex-grow-0 flex-shrink-0 w-[215px] h-12 text-[32px] text-left text-black",
                        "\n          Yousof Mersal\n        "
                    }
                    p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                        "\n          Born on the 4th November 1996\n        "
                    }
                    p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                        "\n          Lives in Odense Danmark\n        "
                    }
                }
                div { class: "flex flex-col justify-between items-start flex-grow-0 flex-shrink-0 h-[133px] w-[250px]",
                    div { class: "flex justify-center items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            width: "16",
                            "fill": "none",
                            "viewBox": "0 0 16 17",
                            height: "17",
                            "preserveAspectRatio": "none",
                            class: "flex-grow-0 flex-shrink-0 w-4 h-4 relative",
                            g { "clip-path": "url(#clip0_1_34)",
                                path {
                                    "d": "M5.15312 1.26875C4.9125 0.687499 4.27812 0.378124 3.67188 0.543749L0.921875 1.29375C0.378125 1.44375 0 1.9375 0 2.5C0 10.2312 6.26875 16.5 14 16.5C14.5625 16.5 15.0563 16.1219 15.2063 15.5781L15.9563 12.8281C16.1219 12.2219 15.8125 11.5875 15.2312 11.3469L12.2312 10.0969C11.7219 9.88437 11.1313 10.0312 10.7844 10.4594L9.52188 12C7.32188 10.9594 5.54063 9.17812 4.5 6.97812L6.04063 5.71875C6.46875 5.36875 6.61562 4.78125 6.40312 4.27187L5.15312 1.27187V1.26875Z",
                                    "fill": "#0294DE"
                                }
                            }
                            defs {
                                clipPath { id: "clip0_1_34",
                                    rect {
                                        width: "16",
                                        "transform": "translate(0 0.5)",
                                        height: "16",
                                        "fill": "white"
                                    }
                                }
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "+45 26787730"
                        }
                    }
                    div { class: "flex justify-center items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            "preserveAspectRatio": "none",
                            "fill": "none",
                            "viewBox": "0 0 16 12",
                            height: "12",
                            width: "16",
                            class: "flex-grow-0 flex-shrink-0",
                            path {
                                "fill": "#0294DE",
                                "d": "M1.5 0C0.671875 0 0 0.671875 0 1.5C0 1.97187 0.221875 2.41562 0.6 2.7L7.4 7.8C7.75625 8.06563 8.24375 8.06563 8.6 7.8L15.4 2.7C15.7781 2.41562 16 1.97187 16 1.5C16 0.671875 15.3281 0 14.5 0H1.5ZM0 3.5V10C0 11.1031 0.896875 12 2 12H14C15.1031 12 16 11.1031 16 10V3.5L9.2 8.6C8.4875 9.13438 7.5125 9.13438 6.8 8.6L0 3.5Z"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            yousofmersal@gmail.com\n          "
                        }
                    }
                    div { class: "flex justify-center items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            width: "16",
                            "viewBox": "0 0 16 17",
                            height: "17",
                            "fill": "none",
                            "preserveAspectRatio": "none",
                            class: "flex-grow-0 flex-shrink-0",
                            path {
                                "d": "M5.43437 13.1124C5.43437 13.1749 5.3625 13.2249 5.27187 13.2249C5.16875 13.2343 5.09688 13.1843 5.09688 13.1124C5.09688 13.0499 5.16875 12.9999 5.25938 12.9999C5.35313 12.9905 5.43437 13.0405 5.43437 13.1124ZM4.4625 12.9718C4.44063 13.0343 4.50313 13.1061 4.59688 13.1249C4.67813 13.1561 4.77187 13.1249 4.79063 13.0624C4.80938 12.9999 4.75 12.928 4.65625 12.8999C4.575 12.878 4.48438 12.9093 4.4625 12.9718ZM5.84375 12.9186C5.75313 12.9405 5.69062 12.9999 5.7 13.0718C5.70937 13.1343 5.79063 13.1749 5.88438 13.153C5.975 13.1311 6.0375 13.0718 6.02812 13.0093C6.01875 12.9499 5.93437 12.9093 5.84375 12.9186ZM7.9 0.943634C3.56563 0.943634 0.25 4.23426 0.25 8.56863C0.25 12.0343 2.43125 14.9999 5.54688 16.0436C5.94688 16.1155 6.0875 15.8686 6.0875 15.6655C6.0875 15.4718 6.07812 14.403 6.07812 13.7468C6.07812 13.7468 3.89062 14.2155 3.43125 12.8155C3.43125 12.8155 3.075 11.9061 2.5625 11.6718C2.5625 11.6718 1.84687 11.1811 2.6125 11.1905C2.6125 11.1905 3.39062 11.253 3.81875 11.9968C4.50312 13.203 5.65 12.8561 6.09688 12.6499C6.16875 12.1499 6.37188 11.803 6.59688 11.5968C4.85 11.403 3.0875 11.1499 3.0875 8.14363C3.0875 7.28426 3.325 6.85301 3.825 6.30301C3.74375 6.09988 3.47813 5.26238 3.90625 4.18113C4.55937 3.97801 6.0625 5.02488 6.0625 5.02488C6.6875 4.84988 7.35938 4.75926 8.025 4.75926C8.69063 4.75926 9.3625 4.84988 9.9875 5.02488C9.9875 5.02488 11.4906 3.97488 12.1438 4.18113C12.5719 5.26551 12.3063 6.09988 12.225 6.30301C12.725 6.85613 13.0312 7.28738 13.0312 8.14363C13.0312 11.1593 11.1906 11.3999 9.44375 11.5968C9.73125 11.8436 9.975 12.3124 9.975 13.0468C9.975 14.0999 9.96562 15.403 9.96562 15.6593C9.96562 15.8624 10.1094 16.1093 10.5063 16.0374C13.6313 14.9999 15.75 12.0343 15.75 8.56863C15.75 4.23426 12.2344 0.943634 7.9 0.943634ZM3.2875 11.7218C3.24687 11.753 3.25625 11.8249 3.30938 11.8843C3.35938 11.9343 3.43125 11.9561 3.47187 11.9155C3.5125 11.8843 3.50312 11.8124 3.45 11.753C3.4 11.703 3.32812 11.6811 3.2875 11.7218ZM2.95 11.4686C2.92813 11.5093 2.95937 11.5593 3.02187 11.5905C3.07187 11.6218 3.13438 11.6124 3.15625 11.5686C3.17812 11.528 3.14687 11.478 3.08437 11.4468C3.02187 11.428 2.97187 11.4374 2.95 11.4686ZM3.9625 12.5811C3.9125 12.6218 3.93125 12.7155 4.00312 12.7749C4.075 12.8468 4.16562 12.8561 4.20625 12.8061C4.24688 12.7655 4.22813 12.6718 4.16563 12.6124C4.09688 12.5405 4.00313 12.5311 3.9625 12.5811ZM3.60625 12.1218C3.55625 12.153 3.55625 12.2343 3.60625 12.3061C3.65625 12.378 3.74062 12.4093 3.78125 12.378C3.83125 12.3374 3.83125 12.2561 3.78125 12.1843C3.7375 12.1124 3.65625 12.0811 3.60625 12.1218Z",
                                "fill": "#0294DE"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            github.com/YousofMersal\n          "
                        }
                    }
                    div { class: "flex justify-center items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 14 14",
                            "fill": "none",
                            "preserveAspectRatio": "none",
                            width: "14",
                            height: "14",
                            class: "flex-grow-0 flex-shrink-0",
                            path {
                                "fill": "#0294DE",
                                "d": "M13 0H0.996875C0.446875 0 0 0.453125 0 1.00938V12.9906C0 13.5469 0.446875 14 0.996875 14H13C13.55 14 14 13.5469 14 12.9906V1.00938C14 0.453125 13.55 0 13 0ZM4.23125 12H2.15625V5.31875H4.23438V12H4.23125ZM3.19375 4.40625C2.52812 4.40625 1.99063 3.86562 1.99063 3.20312C1.99063 2.54063 2.52812 2 3.19375 2C3.85625 2 4.39687 2.54063 4.39687 3.20312C4.39687 3.86875 3.85938 4.40625 3.19375 4.40625ZM12.0094 12H9.93437V8.75C9.93437 7.975 9.91875 6.97813 8.85625 6.97813C7.775 6.97813 7.60938 7.82188 7.60938 8.69375V12H5.53438V5.31875H7.525V6.23125H7.55312C7.83125 5.70625 8.50938 5.15312 9.51875 5.15312C11.6187 5.15312 12.0094 6.5375 12.0094 8.3375V12Z"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            linkedin.com/in/YousofMersal\n          "
                        }
                    }
                    div { class: "flex justify-center items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            width: "17",
                            "preserveAspectRatio": "none",
                            "viewBox": "0 0 17 17",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            height: "17",
                            class: "flex-grow-0 flex-shrink-0",
                            path {
                                "fill": "#0294DE",
                                "d": "M11.5 8.5C11.5 9.19375 11.4625 9.8625 11.3969 10.5H5.60313C5.53438 9.8625 5.5 9.19375 5.5 8.5C5.5 7.80625 5.5375 7.1375 5.60313 6.5H11.3969C11.4656 7.1375 11.5 7.80625 11.5 8.5ZM12.4 6.5H16.2469C16.4125 7.14062 16.5 7.80937 16.5 8.5C16.5 9.19063 16.4125 9.85938 16.2469 10.5H12.4C12.4656 9.85625 12.5 9.1875 12.5 8.5C12.5 7.8125 12.4656 7.14375 12.4 6.5ZM15.9187 5.5H12.2719C11.9594 3.50312 11.3406 1.83125 10.5437 0.7625C12.9906 1.40938 14.9812 3.18438 15.9156 5.5H15.9187ZM11.2594 5.5H5.74062C5.93125 4.3625 6.225 3.35625 6.58437 2.54063C6.9125 1.80313 7.27812 1.26875 7.63125 0.93125C7.98125 0.6 8.27187 0.5 8.5 0.5C8.72812 0.5 9.01875 0.6 9.36875 0.93125C9.72187 1.26875 10.0875 1.80313 10.4156 2.54063C10.7781 3.35313 11.0687 4.35938 11.2594 5.5ZM4.72813 5.5H1.08125C2.01875 3.18438 4.00625 1.40938 6.45625 0.7625C5.65938 1.83125 5.04063 3.50312 4.72813 5.5ZM0.753125 6.5H4.6C4.53437 7.14375 4.5 7.8125 4.5 8.5C4.5 9.1875 4.53437 9.85625 4.6 10.5H0.753125C0.5875 9.85938 0.5 9.19063 0.5 8.5C0.5 7.80937 0.5875 7.14062 0.753125 6.5ZM6.58437 14.4563C6.22187 13.6438 5.93125 12.6375 5.74062 11.5H11.2594C11.0687 12.6375 10.775 13.6438 10.4156 14.4563C10.0875 15.1938 9.72187 15.7281 9.36875 16.0656C9.01875 16.4 8.72812 16.5 8.5 16.5C8.27187 16.5 7.98125 16.4 7.63125 16.0688C7.27812 15.7313 6.9125 15.1969 6.58437 14.4594V14.4563ZM4.72813 11.5C5.04063 13.4969 5.65938 15.1687 6.45625 16.2375C4.00625 15.5906 2.01875 13.8156 1.08125 11.5H4.72813ZM15.9187 11.5C14.9812 13.8156 12.9937 15.5906 10.5469 16.2375C11.3438 15.1687 11.9594 13.4969 12.275 11.5H15.9187Z"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "yousofmersal.com"
                        }
                    }
                }
                div { class: "flex flex-col justify-between items-start flex-grow-0 flex-shrink-0 h-[203px] w-[250px] relative overflow-hidden pl-2.5 py-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "Interests"
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            height: "6",
                            "preserveAspectRatio": "none",
                            "viewBox": "0 0 6 6",
                            "xmlns": "http://www.w3.org/2000/svg",
                            width: "6",
                            "fill": "none",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "cx": "3",
                                "cy": "3.16667",
                                "r": "2",
                                "fill": "#D9D9D9",
                                "stroke": "#0294DE"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Avid Guitarist"
                        }
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            width: "6",
                            "fill": "none",
                            "viewBox": "0 0 6 6",
                            "preserveAspectRatio": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            height: "6",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "fill": "#D9D9D9",
                                "cy": "2.83334",
                                "r": "2",
                                "stroke": "#0294DE",
                                "cx": "3"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            Loves Dungeons and Dragons\n          "
                        }
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 6 5",
                            "preserveAspectRatio": "none",
                            width: "6",
                            "fill": "none",
                            height: "5",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "cy": "2.5",
                                "fill": "#D9D9D9",
                                "cx": "3",
                                "r": "2",
                                "stroke": "#0294DE"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            Raid-lead World of Warcraft\n          "
                        }
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            height: "6",
                            "viewBox": "0 0 6 6",
                            width: "6",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "preserveAspectRatio": "none",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "fill": "#D9D9D9",
                                "cy": "3.16666",
                                "cx": "3",
                                "r": "2",
                                "stroke": "#0294DE"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            Parallel Computation\n          "
                        }
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            "preserveAspectRatio": "none",
                            "viewBox": "0 0 6 6",
                            "fill": "none",
                            width: "6",
                            height: "6",
                            "xmlns": "http://www.w3.org/2000/svg",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "cx": "3",
                                "r": "2",
                                "fill": "#D9D9D9",
                                "cy": "2.83331",
                                "stroke": "#0294DE"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "\n            Reinforcement Learning\n          "
                        }
                    }
                    div { class: "flex justify-start items-center self-stretch flex-grow-0 flex-shrink-0 relative gap-[5px]",
                        svg {
                            width: "6",
                            "fill": "none",
                            height: "5",
                            "preserveAspectRatio": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 6 5",
                            class: "flex-grow-0 flex-shrink-0",
                            circle {
                                "fill": "#D9D9D9",
                                "cx": "3",
                                "cy": "2.5",
                                "stroke": "#0294DE",
                                "r": "2"
                            }
                        }
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Sustainability"
                        }
                    }
                }
                div { class: "flex flex-col justify-between items-start flex-grow-0 flex-shrink-0 h-[203px] w-[250px] relative overflow-hidden p-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "\n          Tool proficiency\n        "
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Rust"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "viewBox": "0 0 6 6",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "r": "2.5",
                                    "cy": "3.16666",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 6",
                                "fill": "none",
                                height: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "r": "2.5",
                                    "cy": "3.16666"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                height: "6",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "3.16666",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 6",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "cy": "3.16666",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "fill": "none",
                                height: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "3.16666",
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "C#"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "preserveAspectRatio": "none",
                                height: "6",
                                width: "6",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cy": "2.83334",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                width: "6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.83334",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                height: "6",
                                "fill": "none",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.83334",
                                    "cx": "3",
                                    "r": "2.5",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                height: "6",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.83334",
                                    "fill": "#0294DE",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "fill": "none",
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.83334",
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "r": "2.5"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Java"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "5",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "2.5",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 5",
                                height: "5",
                                "fill": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.5",
                                    "cx": "3",
                                    "r": "2.5",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "preserveAspectRatio": "none",
                                width: "6",
                                "fill": "none",
                                height: "5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 6 5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "cy": "2.5",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                height: "5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "2.5",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                width: "6",
                                height: "5",
                                "viewBox": "0 0 6 5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2",
                                    "stroke": "#0294DE",
                                    "cy": "2.5",
                                    "cx": "3"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Javascript"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "3.16669"
                                }
                            }
                            svg {
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "r": "2.5",
                                    "cy": "3.16669",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                width: "6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "3.16669",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "3.16669"
                                }
                            }
                            svg {
                                width: "6",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "3.16669",
                                    "r": "2",
                                    "stroke": "#0294DE"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "C++"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                width: "6",
                                "fill": "none",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.83331",
                                    "fill": "#0294DE",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 6",
                                width: "6",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "2.83331",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                height: "6",
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "r": "2.5",
                                    "cy": "2.83331"
                                }
                            }
                            svg {
                                height: "6",
                                width: "6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "cy": "2.83331",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                height: "6",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "stroke": "#0294DE",
                                    "r": "2",
                                    "cx": "3",
                                    "cy": "2.83331"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "SQL"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                height: "5",
                                "viewBox": "0 0 6 5",
                                width: "6",
                                "fill": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.5",
                                    "fill": "#0294DE",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 5",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                height: "5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "r": "2.5",
                                    "cy": "2.5"
                                }
                            }
                            svg {
                                width: "6",
                                height: "5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "viewBox": "0 0 6 5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cy": "2.5",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 5",
                                width: "6",
                                height: "5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "2.5"
                                }
                            }
                            svg {
                                height: "5",
                                "viewBox": "0 0 6 5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                width: "6",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.5",
                                    "cx": "3",
                                    "stroke": "#0294DE",
                                    "r": "2"
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-col justify-between items-start flex-grow-0 flex-shrink-0 h-[132px] w-[250px] relative overflow-hidden p-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "Languages"
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Danish"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "fill": "none",
                                height: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 6 6",
                                width: "6",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "3.16669",
                                    "r": "2.5",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "3.16669"
                                }
                            }
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "viewBox": "0 0 6 6",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "3.16669",
                                    "r": "2.5",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                height: "6",
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "r": "2.5",
                                    "cy": "3.16669",
                                    "cx": "3"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 6",
                                "fill": "none",
                                height: "6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "3.16669",
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "English"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                "viewBox": "0 0 6 6",
                                height: "6",
                                width: "6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "2.83331",
                                    "r": "2.5",
                                    "fill": "#0294DE"
                                }
                            }
                            svg {
                                width: "6",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                height: "6",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "2.83331",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                width: "6",
                                height: "6",
                                "fill": "none",
                                "preserveAspectRatio": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 6 6",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "fill": "#0294DE",
                                    "cx": "3",
                                    "cy": "2.83331"
                                }
                            }
                            svg {
                                "viewBox": "0 0 6 6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "fill": "none",
                                height: "6",
                                "preserveAspectRatio": "none",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "2.83331"
                                }
                            }
                            svg {
                                width: "6",
                                height: "6",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                "viewBox": "0 0 6 6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "fill": "#0294DE",
                                    "cy": "2.83331",
                                    "r": "2.5",
                                    "cx": "3"
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-center self-stretch flex-grow-0 flex-shrink-0 relative",
                        p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                            "Egyptian/Arabic"
                        }
                        div { class: "flex justify-center items-center flex-grow-0 flex-shrink-0 w-[45px] relative overflow-hidden gap-0.5 py-px",
                            svg {
                                width: "6",
                                "preserveAspectRatio": "none",
                                "viewBox": "0 0 6 5",
                                height: "5",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2.5",
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "2.5"
                                }
                            }
                            svg {
                                width: "6",
                                "preserveAspectRatio": "none",
                                height: "5",
                                "viewBox": "0 0 6 5",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "fill": "#0294DE",
                                    "cy": "2.5",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                "fill": "none",
                                height: "5",
                                "viewBox": "0 0 6 5",
                                "preserveAspectRatio": "none",
                                width: "6",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cx": "3",
                                    "cy": "2.5",
                                    "fill": "#0294DE",
                                    "r": "2.5"
                                }
                            }
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                width: "6",
                                "preserveAspectRatio": "none",
                                height: "5",
                                "viewBox": "0 0 6 5",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "r": "2",
                                    "cy": "2.5",
                                    "stroke": "#0294DE",
                                    "cx": "3"
                                }
                            }
                            svg {
                                width: "6",
                                "viewBox": "0 0 6 5",
                                "preserveAspectRatio": "none",
                                "fill": "none",
                                height: "5",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "flex-grow-0 flex-shrink-0",
                                circle {
                                    "cy": "2.5",
                                    "cx": "3",
                                    "r": "2",
                                    "stroke": "#0294DE"
                                }
                            }
                        }
                    }
                }
            }
            svg {
                "viewBox": "0 0 3 1080",
                width: "3",
                height: "1080",
                "xmlns": "http://www.w3.org/2000/svg",
                "preserveAspectRatio": "xMidYMid meet",
                "fill": "none",
                class: "flex-grow-0 flex-shrink-0",
                line {
                    "y2": "1080",
                    "y1": "4.37114e-8",
                    "x1": "1.50006",
                    "x2": "1.50001",
                    "stroke": "#808080",
                    "stroke-width": "2"
                }
            }
            div { class: "flex flex-col justify-start items-start flex-grow-0 flex-shrink-0 h-[1078px] w-[779px] overflow-hidden gap-2.5 pl-[150px] pr-[70px] py-[54px]",
                div { class: "flex flex-col justify-start items-start flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5 pl-2.5 py-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "Experience"
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              Jan. 2018 - Feb. 2019\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base font-semibold text-left text-black",
                                "\n              Freelance Web Programmer\n            "
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              MERN stack developer for small to medium businesses and startups\n            "
                            }
                        }
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              Feb. 2021 - Jan. 2024\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base font-semibold text-left text-black",
                                "\n              Teaching Assistant\n            "
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              University of Southern Denmark\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Teaching Assistant in Concurrent Programming\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Teaching assistant in Introduction to programming\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Teaching Assistant in Database Systems\n            "
                            }
                        }
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              June. 2022 - Mar 2024\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base font-semibold text-left text-black",
                                "\n              Consulting Software Developer\n            "
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              At KB Entertainment Limited\n            "
                            }
                        }
                    }
                }
                div { class: "flex flex-col justify-start items-start flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5 pl-2.5 py-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "Education"
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              Sep. 2022 - Aug 2024\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              M.Sc. In Computer Science\n            "
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              University of Southern Denmark\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "Odense - Denmark"
                            }
                        }
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              Sep. 2019 - Aug 2022\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              B.Sc. In Computer Science\n            "
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              University of Southern Denmark\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "Odense - Denmark"
                            }
                        }
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex justify-start items-center flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-[#0294de]",
                                "\n              Sep. 1014 - June 2018\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "High School"
                            }
                        }
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Stenhus Gymnasium\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Mathematics and Physics\n            "
                            }
                        }
                    }
                }
                div { class: "flex flex-col justify-start items-start flex-grow-0 flex-shrink-0 relative overflow-hidden gap-2.5 pl-2.5 py-2.5",
                    p { class: "flex-grow-0 flex-shrink-0 text-[29px] text-left text-[#0294de]",
                        "\n          Relevant Course Work\n        "
                    }
                    div { class: "flex flex-col justify-center items-start self-stretch flex-grow-0 flex-shrink-0 gap-[5px]",
                        div { class: "flex flex-col justify-center items-start flex-grow-0 flex-shrink-0 relative overflow-hidden pl-10",
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "Database Systems"
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "Dev(Sec)Ops"
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Software Engineering\n            "
                            }
                            p { class: "flex-grow-0 flex-shrink-0 text-base text-left text-black",
                                "\n              Distributed Systems\n            "
                            }
                        }
                    }
                }
            }
        }
    }
    }
}