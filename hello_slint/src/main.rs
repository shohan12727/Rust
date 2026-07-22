use slint::slint;

slint! {
    import { Button, VerticalBox, HorizontalBox, LineEdit } from "std-widgets.slint";

    export component AppWindow inherits Window {
        width: 1000px;
        height: 650px;
        title: "KOS Desktop";
        background: #1E1E2E;

        HorizontalLayout {

            // Left Sidebar
            Rectangle {
                background: #2A2A3C;
                width: 220px;

                VerticalLayout {
                    padding: 20px;
                    spacing: 15px;

                    Text {
                        text: "KOS";
                        color: white;
                        font-size: 30px;
                        font-weight: 700;
                    }

                    Rectangle { height: 2px; background: #555; }

                    Button { text: "🏠 Dashboard"; }
                    Button { text: "📁 Files"; }
                    Button { text: "⚙ Settings"; }
                    Button { text: "📊 Analytics"; }



                    Button { text: "🚪 Exit"; }
                }
            }

            // Main Content
            Rectangle {
                background: #F3F4F6;

                VerticalLayout {
                    padding: 25px;
                    spacing: 20px;

                    Text {
                        text: "Welcome to KOS";
                        font-size: 32px;
                        font-weight: 700;
                        color: #222;
                    }

                    Text {
                        text: "Your first desktop application built with Rust + Slint.";
                        font-size: 18px;
                        color: #555;
                    }

                    HorizontalLayout {
                        spacing: 15px;

                        Rectangle {
                            border-radius: 12px;
                            background: #4F46E5;

                            VerticalLayout {
                                padding: 20px;

                                Text {
                                    text: "Projects";
                                    color: white;
                                    font-size: 20px;
                                }

                                Text {
                                    text: "12";
                                    color: white;
                                    font-size: 42px;
                                }
                            }
                        }

                        Rectangle {
                            border-radius: 12px;
                            background: #059669;

                            VerticalLayout {
                                padding: 20px;

                                Text {
                                    text: "Tasks";
                                    color: white;
                                    font-size: 20px;
                                }

                                Text {
                                    text: "38";
                                    color: white;
                                    font-size: 42px;
                                }
                            }
                        }

                        Rectangle {
                            border-radius: 12px;
                            background: #DC2626;

                            VerticalLayout {
                                padding: 20px;

                                Text {
                                    text: "Messages";
                                    color: white;
                                    font-size: 20px;
                                }

                                Text {
                                    text: "5";
                                    color: white;
                                    font-size: 42px;
                                }
                            }
                        }
                    }

                    Rectangle {
                        border-radius: 12px;
                        background: white;

                        VerticalLayout {
                            padding: 20px;
                            spacing: 15px;

                            Text {
                                text: "Quick Note";
                                font-size: 22px;
                                color: #222;
                            }

                           LineEdit {
    placeholder-text: "Write something...";
}
                            Button {
                                text: "Save";
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    AppWindow::new().unwrap().run().unwrap();
}