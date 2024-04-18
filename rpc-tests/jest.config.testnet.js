const config = {
  testTimeout: 120000,
  reporters: [
    "default",
    [
      "jest-html-reporter", {
      pageTitle: "RPC Test Report",
      outputPath: getReportPath('testnet'),
      includeFailureMsg: true,
    }
    ],
  ],
};

function getReportPath(testEnv) {
  const timestamp = new Date().toISOString().replace(/:/g, "-").slice(0, -5);
  return `./reports/test-report-${testEnv}-${timestamp}.html`;
}

module.exports = config;
