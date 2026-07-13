"""
Main code cell containing a `CodeEdit` text input and a `RenderViewer` output viewer.
"""

from PyQt5.QtWidgets import (
    QVBoxLayout, QLabel, QSizePolicy
)
import MothballSimulationXZ as mxz
import MothballSimulationY as my

from PyQt5.QtCore import Qt, QThread, pyqtSignal, QObject
import FileHandler
from PyQt5.QtGui import QColor, QKeySequence
from BaseCell import Cell, CodeEdit, RenderViewer
from Linters import CodeLinter
from Enums import *
from PyQt5.QtWidgets import QHBoxLayout, QPushButton, QWidget, QComboBox, QShortcut, QLineEdit
import os, json

from TapBruteForcer.main import find_tap_strats
from TapBruteForcer.parser import update_params, get_bf_text
import TapBruteForcer.helper as helper

class Worker(QObject):
    finished = pyqtSignal(list, dict, int)

    def __init__(self, input_str, simulation_type, tap_params={"active" : False}):
        super().__init__()
        self.input_str = input_str
        self.simulation_type = simulation_type
        self.p = None
        self.isrunning = False
        self.tap_params = tap_params

    def run(self):
        self.isrunning = True
        try:
            if self.tap_params["active"]:
                tap_output = find_tap_strats(self.tap_params)
                normal_input_str = helper.sanitize_mothball_cmd(self.tap_params["mothball"])

                self.p = mxz.PlayerSimulationXZ()
                self.p.simulate(normal_input_str, suppress_exception=True)
                output = self.p.output + [(8, ("",)), (4, ("---------- Tap Strats Below ----------",)), (8, ("",))] + tap_output
                self.finished.emit(output, self.p.macros, 1)
            elif self.simulation_type == CellType.XZ:
                self.p = mxz.PlayerSimulationXZ()
                self.p.simulate(self.input_str, suppress_exception=False)
                self.finished.emit(self.p.output, self.p.macros, 1)
            elif self.simulation_type == CellType.Y:
                self.p = my.PlayerSimulationY()
                a = self.p.simulate(self.input_str, suppress_exception= False)
                self.finished.emit(self.p.output, {}, 1)
        except Exception as e:
            self.finished.emit([(ExpressionType.GENERAL_LABEL, (f"Error occurred: {str(e)}",))], {}, 0)
        self.isrunning = False

    def cancel(self):
        if self.p:
            self.p.stop_execution()

class SimulationSection(Cell):
    "Mothball Code Cell, `CodeEdit` as the input field, `RenderViewer` as the output viewer. The actual highlighting is done here, and the highlighting logic is computed in its linter `self.linter`."
    def __init__(self, parent, generalOptions: dict, colorOptions: dict, textOptions: dict, remove_callback, add_callback, move_callback, change_callback, copy_callback, mode: CellType, tap_cell=False):
        super().__init__(parent, generalOptions, colorOptions, textOptions, remove_callback, add_callback, move_callback, change_callback, copy_callback, mode)
        self.mode = mode
        self.words = []
        self.raw_output = []
        self.linter = CodeLinter(generalOptions, colorOptions, textOptions, mode)
        self.mc_macros_folders: dict = generalOptions.get("Macro Folders", {"default":FileHandler.getMacros()})
        self.macros = {}
        self.worker = None
        self.t = None
        self.p = parent # The main Mothball instance 

        content_layout = QVBoxLayout()

        tophlayout = QHBoxLayout()
        tophlayout.setAlignment(Qt.AlignmentFlag.AlignLeft)
        tophlayout.setContentsMargins(0,0,0,0)
        content_layout.addLayout(tophlayout)

        if mode == CellType.XZ:
            self.input_label = QLabel("Input (XZ):")
        elif mode == CellType.Y:
            self.input_label = QLabel("Input (Y):")
        self.input_label.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        tophlayout.addWidget(self.input_label)

        if tap_cell:
            self.goal_type_button = QPushButton("Goal Type: Min and max")
            self.goal_type_button.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
            self.goal_type_button.setFixedWidth(150)
            self.goal_type_button.clicked.connect(self.switch_goal_type)
            tophlayout.addWidget(self.goal_type_button)

            self.axis_button = QPushButton("Axis: XZ")
            self.axis_button.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
            self.axis_button.setFixedWidth(80)
            self.axis_button.clicked.connect(self.switch_axis)
            tophlayout.addWidget(self.axis_button)

            self.init_taps_button = QPushButton("Taps")
            self.init_taps_button.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
            self.init_taps_button.setFixedWidth(80)
            self.init_taps_button.clicked.connect(self.init_taps)
            tophlayout.addWidget(self.init_taps_button)

            self.tap_params = {
                "active" : False,

                "goal_type" : "minmax",
                "axis" : "XZ",
                "mothball" : "",

                "n" : 0,
                "f" : "0",
                "do_frange" : False,
                "fstart" : 0,
                "fend" : 0,
                "fstep" : 0.05,
                "xmin" : float("-inf"),
                "xmax" : float("inf"),
                "zmin" : float("-inf"),
                "zmax" : float("inf"),
                "xtarget" : 0,
                "xerror" : float("inf"),
                "ztarget" : 0,
                "zerror" : float("inf"),
                "packages" : "",
                "corners" : "",
                "sortby" : "",
                "version" : "1.8",
                "dp" : 6,
                "slip" : "Not implemented",
            }
        else:
            self.tap_params = {"active" : False}

        self.edit_or_save_name_button = QPushButton("🖉")
        self.edit_or_save_name_button.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        self.edit_or_save_name_button.setFixedWidth(40)
        self.edit_or_save_name_button.clicked.connect(self.editCellName)
        tophlayout.addWidget(self.edit_or_save_name_button)

        self.cell_name = QLabel("Unnamed Cell")
        tophlayout.addWidget(self.cell_name)

        self.edit_name_field = QLineEdit("Unnamed Cell")
        self.edit_name_field.hide()
        self.edit_name_field.editingFinished.connect(self.saveCellName)
        tophlayout.addWidget(self.edit_name_field)


        self.input_field = CodeEdit(generalOptions, colorOptions, textOptions, self)
        self.input_field.textChanged.connect(self.highlight)
        self.input_field.textChanged.connect(change_callback)
        self.input_field.setMatchedBraceForegroundColor(QColor("lime"))
        self.input_field.setUnmatchedBraceForegroundColor(QColor('red'))
        self.input_field.setUnmatchedBraceBackgroundColor(QColor("#4F4F4F"))
        self.input_field.setMatchedBraceBackgroundColor(QColor("#4F4F4F"))

        self.bracket_colors = {0:Style.NEST0, 1:Style.NEST1, 2:Style.NEST2}

        content_layout.addWidget(self.input_field)

        self.output_and_artifacts = QWidget()
        hlayout = QHBoxLayout()
        hlayout.setAlignment(Qt.AlignmentFlag.AlignLeft)
        self.output_and_artifacts.setLayout(hlayout)

        self.output_label = QLabel("Output:")
        self.output_label.setAlignment(Qt.AlignmentFlag.AlignLeft)
        self.output_label.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        hlayout.addWidget(self.output_label)
        

        self.macros_label = QLabel("Macros:")
        self.macros_label.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        hlayout.addWidget(self.macros_label)
        self.macros_label.hide()
        self.artifacts_list = QComboBox()
        hlayout.addWidget(self.artifacts_list)
        self.artifacts_list.hide()
        content_layout.addWidget(self.output_and_artifacts)
        
        self.path_selection = QComboBox()
        self.path_selection.addItems(self.mc_macros_folders.keys())
        self.path_selection.hide()
        hlayout.addWidget(self.path_selection)


        self.view_macro = QPushButton("View")
        self.view_macro.hide()
        hlayout.addWidget(self.view_macro)
        self.view_macro.clicked.connect(self.viewMacro)



        self.save_to_folder_button = QPushButton("Save")
        self.save_to_folder_button.hide()
        hlayout.addWidget(self.save_to_folder_button)
        self.save_to_folder_button.clicked.connect(self.saveToFolder)

        # Popup for save success
        self.message_label = QLabel("")
        self.message_label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.message_label.setWindowFlags(Qt.WindowType.ToolTip)
        self.message_label.hide()
        hlayout.addWidget(self.message_label)

        self.output_field = RenderViewer(generalOptions, colorOptions, textOptions, self)
        self.output_field.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)
        self.output_field.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        self.output_field.verticalScrollBar().setVisible(False)
        self.output_field.textChanged.connect(self.adjust_output_height)

        content_layout.addWidget(self.output_field)

        content_layout.addStretch(1)

        self.main_layout.addLayout(content_layout)
        self.setLayout(self.main_layout)

        self.run_button.clicked.connect(self.run_simulation)

        self.adjust_output_height()

        self.run_shortcut = QShortcut(QKeySequence("Ctrl+R"), self)
        self.run_shortcut.setContext(Qt.ShortcutContext.WidgetWithChildrenShortcut)
        self.run_shortcut.activated.connect(self.run_simulation)

    def load_tap_bf_params(self):
        display_text = {
        "minmax" : "Min and max" ,
        "target" : "Target and error",
        "mothball" : "Mothball",
        }

        goal_type = self.tap_params["goal_type"]
        axis = self.tap_params["axis"]
        self.goal_type_button.setText(f"Goal Type: {display_text[goal_type]}")
        self.axis_button.setText(f"Axis: {axis}")

    def switch_goal_type(self):
        goal_types = {"minmax" : "target", "target" : "mothball", "mothball" : "minmax"}
        self.tap_params["goal_type"] = goal_types[self.tap_params["goal_type"]]
        display_text = {
        "minmax" : "Min and max" ,
        "target" : "Target and error",
        "mothball" : "Mothball",
        }

        goal_type = self.tap_params["goal_type"]
        self.goal_type_button.setText(f"Goal Type: {display_text[goal_type]}")

        curr_text = self.input_field.text()
        update_params(self, strict=False)
        self.input_field.setText(get_bf_text(curr_text, self.tap_params))

    def switch_axis(self):
        axes = {"XZ" : "X", "X" : "Z", "Z" : "XZ"}
        self.tap_params["axis"] = axes[self.tap_params["axis"]]
        axis = self.tap_params["axis"]
        self.axis_button.setText(f"Axis: {axis}")

        curr_text = self.input_field.text()
        update_params(self, strict=False)
        self.input_field.setText(get_bf_text(curr_text, self.tap_params))

    def init_taps(self):
        curr_text = self.input_field.text()

        if self.tap_params["active"]:
            try:
                update_params(self)
                self.input_field.setText(curr_text.partition("-----")[0].rstrip("\n"))
            except Exception as e:
                self.output_field.renderTextfromOutput(self.linter, [(ExpressionType.GENERAL_LABEL, (f"Error occurred: {str(e)}",))])
                return

            self.tap_params["active"] = False
            return
        else:
            self.input_field.setText(get_bf_text(curr_text, self.tap_params))
            self.tap_params["active"] = True

    def editCellName(self):
        self.cell_name.hide()
        self.edit_name_field.show()
        self.edit_or_save_name_button.clicked.disconnect()
        self.edit_or_save_name_button.clicked.connect(self.saveCellName)
        self.edit_name_field.setText(self.cell_name.text())

    def saveCellName(self):
        self.edit_name_field.hide()
        self.cell_name.show()
        self.edit_or_save_name_button.clicked.disconnect()
        self.edit_or_save_name_button.clicked.connect(self.editCellName)
        self.cell_name.setText(self.edit_name_field.text())

    def viewMacro(self):
        file = self.artifacts_list.currentText()
        _ , ext = os.path.splitext(file)
        if ext == '.json':
            exttype = MacroFileExtension.CYV_JSON
        elif ext == '.csv':
            exttype = MacroFileExtension.MPK_CSV
        
        self.p.openMacroViewer(file,self.macros[file],exttype)

    def saveToFolder(self):
        selected_folder_alias = self.path_selection.currentText()
        selected_folder = self.mc_macros_folders[selected_folder_alias]

        if os.path.exists(selected_folder):
            file = self.artifacts_list.currentText()
            _ , ext = os.path.splitext(file)
            with open(os.path.join(selected_folder, file), "w") as f:
                if ext == '.json':
                    json.dump(self.macros[file], f)
                    self.message_label.setText(f"Saved to {selected_folder_alias} path")
                    self.message_label.setStyleSheet("background-color: #dff0d8; color: #3c763d; border: 1px solid #3c763d; padding: 4px;")
                    self.message_label.show()
                elif ext == '.csv':
                    f.write(self.macros[file])
                    self.message_label.setText(f"Saved to {selected_folder_alias} path")
                    self.message_label.setStyleSheet("background-color: #dff0d8; color: #3c763d; border: 1px solid #3c763d; padding: 4px;")
                    self.message_label.show()
        else:
            self.message_label.setText(f"The {selected_folder_alias} path doesn't exist")
            self.message_label.setStyleSheet("background-color: " + "#dff0d8" "; color: "+"#763c3c"+"; border: 1px solid "+"#763c3c"+"; padding: 4px;")
            self.message_label.show()

    def highlight(self):
        "Get tokens to lint from `self.linter`, an object of `Linters.CodeLinter`, and colorize."
        text = self.input_field.text()
        self.input_field.SendScintilla(self.input_field.SCI_STARTSTYLING, 0, len(text))
        tokens = self.linter.lintTexttoTokens(text)
        for token, style in tokens:
            self.input_field.SendScintilla(self.input_field.SCI_SETSTYLING, len(token), style)

    def adjust_output_height(self):
        "Set height based on height needed to display all content without scrolling."
        doc = self.output_field.document()
        margins = self.output_field.contentsMargins()
        height = int(doc.size().height()) + margins.top() + margins.bottom()
        self.output_field.setFixedHeight(height)

    def run_simulation(self):
        "Execute the Mothball code and show its output."
        text = self.input_field.text()
        if self.tap_params["active"]:
            update_params(self)

        self.t = QThread()
        self.worker = Worker(text, self.mode, tap_params=self.tap_params)
        self.worker.moveToThread(self.t)

        self.t.started.connect(self.worker.run)
        self.worker.finished.connect(self.onSimulationCompletion)
        self.worker.finished.connect(self.t.quit)
        self.worker.finished.connect(self.worker.deleteLater)
        self.t.finished.connect(self.t.deleteLater)

        self.run_button.clicked.disconnect()
        self.run_shortcut.activated.disconnect()
        self.run_button.clicked.connect(self.cancel)

        self.setStatus(self.RUNNING)
        self.t.start()

    def cancel(self):
        if self.worker:
            self.worker.cancel()
        self.run_button.clicked.disconnect()
        self.run_button.clicked.connect(self.run_simulation)
        self.run_shortcut.activated.connect(self.run_simulation)
        

    def onSimulationCompletion(self, output, macros, result: int):
        "result = 1 means success, result = 0 means failure"
        self.output_field.renderTextfromOutput(self.linter, output)
        self.raw_output = output
        
        if self.mode == CellType.XZ:
            if macros:
                self.artifacts_list.clear()
                self.macros_label.show()
                self.artifacts_list.show()
                self.view_macro.show()
                self.path_selection.show()
                self.save_to_folder_button.show()
                for name, artifact in macros.items():
                    self.artifacts_list.addItem(name)
                self.macros = macros

        self.run_button.clicked.disconnect()
        self.run_button.clicked.connect(self.run_simulation)
        self.run_shortcut.activated.connect(self.run_simulation)
        if result: 
            self.setStatus(self.SUCCESS)
        else: 
            self.setStatus(self.ERROR)
    
    def getCellData(self):
        data = {
            "name": self.cell_name.text(),
            "cell_type": self.cellType,
            "code": self.input_field.text(),
            "exec_time": "None",
            "has_changed": False,
            "raw_output": self.raw_output,
            "tap_params" : self.tap_params
        }
        return data
    
    def setupCell(self, data):
        if not all([x in ("cell_type", "name","code","exec_time","has_changed","raw_output","tap_params") for x in data]):
            return
        self.input_field.setText(data['code'].rstrip())
        self.cell_name.setText(data['name'])
        self.output_field.renderTextfromOutput(self.linter, data['raw_output'])
        self.raw_output = data['raw_output']
        self.tap_params = data['tap_params']

        self.load_tap_bf_params()
    
    def resizeEvent(self, event):
        self.adjust_output_height()
        super().resizeEvent(event)
    
    def closeEvent(self, a0):
        return super().closeEvent(a0)