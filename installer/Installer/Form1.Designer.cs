namespace Installer
{
    partial class Window
    {
        /// <summary>
        ///  Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        ///  Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if(disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        ///  Required method for Designer support - do not modify
        ///  the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Window));
            InstallButton = new Button();
            ResponseLabel = new Label();
            Paragraph = new Label();
            SuspendLayout();
            // 
            // InstallButton
            // 
            InstallButton.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            InstallButton.Location = new Point(346, 129);
            InstallButton.Margin = new Padding(3, 2, 3, 2);
            InstallButton.Name = "InstallButton";
            InstallButton.Size = new Size(82, 22);
            InstallButton.TabIndex = 0;
            InstallButton.Text = "Install";
            InstallButton.UseVisualStyleBackColor = true;
            InstallButton.Click += InstallButton_Click;
            // 
            // ResponseLabel
            // 
            ResponseLabel.AutoSize = true;
            ResponseLabel.Location = new Point(12, 108);
            ResponseLabel.Name = "ResponseLabel";
            ResponseLabel.Size = new Size(0, 15);
            ResponseLabel.TabIndex = 1;
            // 
            // Paragraph
            // 
            Paragraph.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            Paragraph.Location = new Point(12, 9);
            Paragraph.Name = "Paragraph";
            Paragraph.Size = new Size(416, 114);
            Paragraph.TabIndex = 2;
            Paragraph.Text = resources.GetString("Paragraph.Text");
            // 
            // Window
            // 
            AutoScaleDimensions = new SizeF(7F, 15F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(440, 162);
            Controls.Add(Paragraph);
            Controls.Add(ResponseLabel);
            Controls.Add(InstallButton);
            FormBorderStyle = FormBorderStyle.FixedSingle;
            Margin = new Padding(3, 2, 3, 2);
            MaximizeBox = false;
            MinimizeBox = false;
            Name = "Window";
            Text = "Gaben Installer";
            ResumeLayout(false);
            PerformLayout();
        }

        #endregion

        private Button InstallButton;
        private Label ResponseLabel;
        private Label Paragraph;
    }
}
