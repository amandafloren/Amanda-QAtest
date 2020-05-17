import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import java.nio.file.Files as Files
import java.nio.file.Path as Path
import java.nio.file.Paths as Paths
import javax.imageio.ImageIO as ImageIO
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import ru.yandex.qatools.ashot.AShot as AShot
import ru.yandex.qatools.ashot.Screenshot as Screenshot
import ru.yandex.qatools.ashot.coordinates.WebDriverCoordsProvider as WebDriverCoordsProvider
import ru.yandex.qatools.ashot.shooting.ShootingStrategies as ShootingStrategies

// take screenshot of entire web page , to be continued, not suitable element while screenshot
// take screenshot of entire web page
WebUI.openBrowser('')

//WebUI.setViewPortSize(375, 667)
WebUI.maximizeWindow()

def topPageUrl = 'https://www.online-calculator.com/full-screen-calculator/'

WebUI.navigateToUrl(topPageUrl)

WebUI.delay(3)

WebUI.verifyElementPresent(findTestObject('Answer/Page_Full Screen Calculator - Online Calculator/fullscreen'), 10, FailureHandling.CONTINUE_ON_FAILURE)

Path projectDir = Paths.get(RunConfiguration.getProjectDir())

Path reportDir = projectDir.resolve('Screenshots')

Files.createDirectories(reportDir)

Path pngFile = reportDir.resolve('screenshot-way2.png')

WebDriver driver = DriverFactory.getWebDriver()

takeEntirePage(driver, pngFile.toFile(), 900)

WebUI.closeBrowser()

void takeEntirePage(WebDriver webDriver, File file, Integer timeout = 900) {
    Screenshot screenshot = new AShot().coordsProvider(new WebDriverCoordsProvider()).shootingStrategy(ShootingStrategies.viewportPasting(
            timeout)).takeScreenshot(webDriver)

    ImageIO.write(screenshot.getImage(), 'PNG', file)
}

