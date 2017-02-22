'use strict';
import * as vscode from 'vscode';
import * as sh from 'shelljs';
import * as child_process from 'child_process';
import * as fs from 'fs';
import * as strip_ansi from 'strip-ansi';

class Rhq {
  clone_option: string;

  constructor() {
    this.clone_option = vscode.workspace.getConfiguration('rhq').get('cloneOption', '');
  }

  command_clone() {
    this.clone();
  }

  command_open() {
    this.open(false);
  }

  command_open_in_new_window() {
    this.open(true);
  }

  private clone() {
    if (!this.check_available()) {
      return;
    }

    const onResolve = (query) => {
      if (query === undefined || query === "") {
        return;
      }

      let args = ['clone', query, '--arg="' + this.clone_option + '"'];
      let proc = child_process.spawn('rhq', args);
      let out_ch = vscode.window.createOutputChannel('rhq');
      out_ch.show(true);

      proc.stdout.on('data', (data) => {
        out_ch.append(strip_ansi(data.toString()));
      });

      proc.stderr.on('data', (data) => {
        out_ch.append(strip_ansi(data.toString()));
      });

      proc.on('close', (code) => {
        out_ch.appendLine('rhq process finished with code ' + code);
      });
    };

    const onReject = (reason) => {
      vscode.window.showWarningMessage(reason.toString());
    };

    vscode.window.showInputBox().then(onResolve, onReject);
  }

  private open(in_newwindow: boolean) {
    if (!this.check_available()) {
      return;
    }

    child_process.exec('rhq list', (err, stdout, stderr) => {
      if (err) {
        vscode.window.showInformationMessage(err.name + ": " + err.message);
      }

      let candidates = stdout.split("\n");

      const onResolve = (selected) => {
        if (selected === undefined || selected === "") {
          return;
        }
        if (!fs.existsSync(selected)) {
          return;
        }
        let uri = vscode.Uri.parse(selected);
        vscode.commands.executeCommand('vscode.openFolder', uri, in_newwindow);
      };

      const onReject = (reason) => {
        vscode.window.showWarningMessage(reason.toString());
      };

      vscode.window.showQuickPick(candidates).then(onResolve, onReject);
    });
  }

  private dump_error(reason) {
    vscode.window.showWarningMessage(reason.toString());
  }

  private check_available(): boolean {
    if (!sh.which('rhq') == true) {
      vscode.window.showErrorMessage("'rhq' is not installed.");
      return false;
    }
    return true;
  }
}

export function activate(context: vscode.ExtensionContext) {
  function register(name, f) {
    context.subscriptions.push(
    vscode.commands.registerCommand(name, f));
  }

  let rhq = new Rhq();
  register("extension.rhqClone", () => rhq.command_clone());
  register("extension.rhqOpen", () => rhq.command_open());
  register("extension.rhqOpenInNewWindow", () => rhq.command_open_in_new_window());
}

export function deactivate() {
}